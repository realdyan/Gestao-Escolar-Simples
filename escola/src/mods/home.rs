use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;
use crate::db;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_setup = conn.clone();

    // Carrega a lista de escolas para o seletor
    carregar_selecao(&ui, &conn_setup.borrow());

    // Configura o clique na escola
    ui.on_selecionar_escola(move |id, nome| {
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_id_escola_ativa(id);
            ui.set_nome_escola_ativa(nome.clone());
            println!("🏫 Contexto alterado para: {}", nome);
        }
    });
}

pub fn carregar_selecao(ui: &crate::MainWindow, conn: &Connection) {
    if let Ok(escolas) = db::listar_escolas(conn) {
        let rows: Vec<crate::EscolaRow> = escolas.into_iter().map(|e| crate::EscolaRow {
            id: e.id_escola.unwrap_or(0),
            nome: e.nome_escola.into(),
            email: e.email.into(),
            telefone: e.telefone.into(),
            endereco: e.endereco.into(),
            website: e.website.into(),
        }).collect();
        ui.set_lista_escolas_selecao(ModelRc::from(Rc::new(VecModel::from(rows))));
    }
}