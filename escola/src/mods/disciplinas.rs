use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    // SALVAR (Novo ou Edição)
    ui.on_disciplina_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let id = ui.get_disciplina_form_id();
            let modo = ui.get_disciplina_modo().to_string();
            
            let disc = db::Disciplina {
                id_disciplina: if id > 0 { Some(id) } else { None },
                id_escola,
                nome_disciplina: ui.get_disciplina_form_nome().to_string(),
                descricao: ui.get_disciplina_form_descricao().to_string(),
                carga_horaria: ui.get_disciplina_form_carga().to_string().parse().unwrap_or(0),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_disciplina(&conn_save.borrow(), id, &disc)
            } else {
                db::inserir_disciplina(&conn_save.borrow(), &disc).map(|_| ())
            };

            if res.is_ok() {
                ui.set_disciplina_modo("listar".into());
                carregar(&ui, &conn_save.borrow(), id_escola);
            }
        }
    });

    // EDITAR (Preencher formulário)
    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_disciplina_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            if let Ok(discs) = db::listar_disciplinas(&conn_edit.borrow(), id_escola) {
                if let Some(d) = discs.into_iter().find(|x| x.id_disciplina == Some(id)) {
                    ui.set_disciplina_form_id(id);
                    ui.set_disciplina_form_nome(d.nome_disciplina.into());
                    ui.set_disciplina_form_descricao(d.descricao.into());
                    ui.set_disciplina_form_carga(d.carga_horaria.to_string().into());
                    ui.set_disciplina_modo("editar".into());
                }
            }
        }
    });

    // ELIMINAR
    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_disciplina_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            if db::remover_disciplina(&conn_del.borrow(), id).is_ok() {
                carregar(&ui, &conn_del.borrow(), id_escola);
            }
        }
    });
}

pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32) {
    if let Ok(discs) = db::listar_disciplinas(conn, id_escola) {
        let rows: Vec<crate::DisciplinaRow> = discs.into_iter().map(|d| crate::DisciplinaRow {
            id: d.id_disciplina.unwrap_or(0), 
            nome: d.nome_disciplina.into(),
            descricao: d.descricao.into(), 
            carga_horaria: d.carga_horaria,
        }).collect();
        ui.set_disciplinas_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
    }
}