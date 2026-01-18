use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    ui.on_professor_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id = ui.get_professor_form_id();
            let id_escola = ui.get_id_escola_ativa();
            let modo = ui.get_professor_modo().to_string();
            
            let prof = db::Professor {
                id_professor: if id > 0 { Some(id) } else { None },
                id_escola,
                nome_professor: ui.get_professor_form_nome().to_string(),
                nome_completo: ui.get_professor_form_nome_completo().to_string(),
                email: ui.get_professor_form_email().to_string(),
                telefone: ui.get_professor_form_telefone().to_string(),
                endereco: ui.get_professor_form_endereco().to_string(),
                data_nascimento: ui.get_professor_form_data_nasc().to_string(),
                cpf: ui.get_professor_form_cpf().to_string(),
                data_contratacao: ui.get_professor_form_data_contrat().to_string(),
                salario_atual: ui.get_professor_form_salario().to_string().parse().unwrap_or(0.0),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_professor(&conn_save.borrow(), id, &prof)
            } else {
                db::inserir_professor(&conn_save.borrow(), &prof).map(|_| ())
            };

            if res.is_ok() {
                ui.set_professor_modo("listar".into());
                carregar(&ui, &conn_save.borrow(), id_escola);
            }
        }
    });

    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_professor_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            match db::listar_professores(&conn_edit.borrow(), ui.get_id_escola_ativa()) {
                Ok(profs) => {
                    if let Some(p) = profs.into_iter().find(|x| x.id_professor == Some(id)) {
                        ui.set_professor_form_id(id);
                        ui.set_professor_form_nome(p.nome_professor.into());
                        ui.set_professor_form_nome_completo(p.nome_completo.into());
                        ui.set_professor_form_email(p.email.into());
                        ui.set_professor_form_telefone(p.telefone.into());
                        ui.set_professor_form_endereco(p.endereco.into());
                        ui.set_professor_form_cpf(p.cpf.into());
                        ui.set_professor_form_data_nasc(p.data_nascimento.into());
                        ui.set_professor_form_data_contrat(p.data_contratacao.into());
                        ui.set_professor_form_salario(p.salario_atual.to_string().into());
                        ui.set_professor_modo("editar".into());
                    }
                }
                Err(e) => eprintln!("❌ Erro ao buscar professor: {:?}", e),
            }
        }
    });

    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_professor_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            if db::remover_professor(&conn_del.borrow(), id).is_ok() {
                carregar(&ui, &conn_del.borrow(), id_escola);
            }
        }
    });
    
    // ✅ REMOVA temporariamente os callbacks de filtro avançado para compilar
    // Vamos implementar depois que os alunos funcionarem
}

pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32) {
    match db::listar_professores(conn, id_escola) {
        Ok(profs) => {
            let rows: Vec<crate::ProfessorRow> = profs.into_iter().map(|p| crate::ProfessorRow {
                id: p.id_professor.unwrap_or(0),
                nome: p.nome_professor.into(),
                email: p.email.into(),
                cpf: p.cpf.into(),
                telefone: p.telefone.into(),
                salario: p.salario_atual as f32,
            }).collect();
            ui.set_professores_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
        }
        Err(e) => eprintln!("❌ Erro ao carregar professores: {:?}", e),
    }
}