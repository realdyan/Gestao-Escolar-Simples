use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    // ==========================================
    // 1. SALVAR (NOVO OU EDIÇÃO)
    // ==========================================
    ui.on_escola_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id = ui.get_escola_form_id();
            let modo = ui.get_escola_modo().to_string();
            
            let escola = db::Escola {
                id_escola: if id > 0 { Some(id) } else { None },
                nome_escola: ui.get_escola_form_nome().to_string(),
                email: ui.get_escola_form_email().to_string(),
                telefone: ui.get_escola_form_telefone().to_string(),
                endereco: ui.get_escola_form_endereco().to_string(),
                website: ui.get_escola_form_website().to_string(),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_escola(&conn_save.borrow(), id, &escola)
            } else {
                db::inserir_escola(&conn_save.borrow(), &escola).map(|_| ())
            };

            match res {
                Ok(_) => {
                    let operacao = if modo == "editar" { "atualizada" } else { "criada" };
                    println!("✅ Escola {} com sucesso (ID: {})", operacao, id);
                    
                    ui.set_escola_modo("listar".into());
                    carregar(&ui, &conn_save.borrow());
                    // Atualiza também o seletor da Home
                    crate::mods::home::carregar_selecao(&ui, &conn_save.borrow());
                }
                Err(e) => {
                    eprintln!("❌ Erro ao salvar escola: {:?}", e);
                }
            }
        }
    });

    // ==========================================
    // 2. EDITAR (CARREGAR FORMULÁRIO)
    // ==========================================
    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_escola_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            match db::buscar_escola_por_id(&conn_edit.borrow(), id) {
                Ok(escola) => {
                    println!("📝 Carregando escola para edição (ID: {})", id);
                    
                    // CRÍTICO: Preenche TODOS os campos do formulário
                    ui.set_escola_form_id(id);
                    ui.set_escola_form_nome(escola.nome_escola.into());
                    ui.set_escola_form_email(escola.email.into());
                    ui.set_escola_form_telefone(escola.telefone.into());
                    ui.set_escola_form_endereco(escola.endereco.into());
                    ui.set_escola_form_website(escola.website.into());
                    ui.set_escola_modo("editar".into());
                }
                Err(e) => {
                    eprintln!("❌ Erro ao buscar escola (ID: {}): {:?}", id, e);
                }
            }
        }
    });

    // ==========================================
    // 3. DELETAR
    // ==========================================
    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_escola_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            match db::remover_escola(&conn_del.borrow(), id) {
                Ok(_) => {
                    println!("🗑️  Escola removida (ID: {})", id);
                    carregar(&ui, &conn_del.borrow());
                    crate::mods::home::carregar_selecao(&ui, &conn_del.borrow());
                }
                Err(e) => {
                    eprintln!("❌ Erro ao deletar escola (ID: {}): {:?}", id, e);
                }
            }
        }
    });
}

// ==========================================
// 4. CARREGAR (SINCRONIZAÇÃO BANCO -> UI)
// ==========================================
pub fn carregar(ui: &crate::MainWindow, conn: &Connection) {
    match db::listar_escolas(conn) {
        Ok(escolas) => {
            let total = escolas.len();
            
            let rows: Vec<crate::EscolaRow> = escolas.into_iter().map(|e| crate::EscolaRow {
                id: e.id_escola.unwrap_or(0),
                nome: e.nome_escola.into(),
                email: e.email.into(),
                telefone: e.telefone.into(),
                endereco: e.endereco.into(),
                website: e.website.into(),
            }).collect();
            
            ui.set_escolas_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
            println!("📊 Escolas carregadas: {} registro(s)", total);
        }
        Err(e) => {
            eprintln!("❌ Erro ao carregar escolas: {:?}", e);
        }
    }
}