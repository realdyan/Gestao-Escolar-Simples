use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    ui.on_curso_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id = ui.get_curso_form_id();
            let modo = ui.get_curso_modo().to_string();
            let id_escola = ui.get_id_escola_ativa();

            let curso = db::Curso {
                id_curso: if id > 0 { Some(id) } else { None },
                id_escola,
                nome_curso: ui.get_curso_form_nome().to_string(),
                descricao: ui.get_curso_form_descricao().to_string(),
                duracao_horas: ui.get_curso_form_duracao().to_string().parse().unwrap_or(0),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_curso(&conn_save.borrow(), id, &curso)
            } else {
                db::inserir_curso(&conn_save.borrow(), &curso).map(|_| ())
            };

            if res.is_ok() {
                ui.set_curso_modo("listar".into());
                carregar(&ui, &conn_save.borrow(), id_escola);
            }
        }
    });

    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_curso_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            if let Ok(cursos) = db::listar_cursos(&conn_edit.borrow(), id_escola) {
                if let Some(c) = cursos.into_iter().find(|x| x.id_curso == Some(id)) {
                    ui.set_curso_form_id(id);
                    ui.set_curso_form_nome(c.nome_curso.into());
                    ui.set_curso_form_duracao(c.duracao_horas.to_string().into());
                    ui.set_curso_modo("editar".into());
                }
            }
        }
    });

    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_curso_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let _ = db::remover_curso(&conn_del.borrow(), id);
            carregar(&ui, &conn_del.borrow(), id_escola);
        }
    });
}

pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32) {
    if let Ok(cursos) = db::listar_cursos(conn, id_escola) {
        let rows: Vec<crate::CursoRow> = cursos.into_iter().map(|c| crate::CursoRow {
            id: c.id_curso.unwrap_or(0), 
            nome: c.nome_curso.into(),
            descricao: c.descricao.into(), 
            duracao_horas: c.duracao_horas,
        }).collect();
        ui.set_cursos_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
    }
}