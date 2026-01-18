use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    // SALVAR NOVA MATRÍCULA
    ui.on_matricula_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let id_aluno = ui.get_matricula_form_id_aluno().to_string().parse().unwrap_or(0);
            let id_turma = ui.get_matricula_form_id_turma().to_string().parse().unwrap_or(0);
            let data = ui.get_matricula_form_data().to_string();

            if db::inserir_matricula(&conn_save.borrow(), id_escola, id_aluno, id_turma, &data).is_ok() {
                ui.set_matricula_modo("listar".into());
                carregar(&ui, &conn_save.borrow(), id_escola);
            }
        }
    });

    // ELIMINAR MATRÍCULA (Ativa a função remover_matricula no db.rs)
    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_matricula_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let _ = db::remover_matricula(&conn_del.borrow(), id);
            carregar(&ui, &conn_del.borrow(), id_escola);
        }
    });
}

pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32) {
    if let Ok(matriculas) = db::listar_matriculas(conn, id_escola) {
        let rows: Vec<crate::MatriculaRow> = matriculas.into_iter().map(|m| crate::MatriculaRow {
            id: m.id_matricula,
            aluno_nome: m.nome_aluno.into(),
            turma_nome: m.nome_turma.into(),
            data: m.data_matricula.into(),
            status: m.status.into(),
        }).collect();
        ui.set_matriculas_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
    }
}