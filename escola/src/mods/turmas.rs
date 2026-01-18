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
    ui.on_turma_salvar(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let id = ui.get_turma_form_id();
            let modo = ui.get_turma_modo().to_string();
            let id_escola = ui.get_id_escola_ativa(); 
            
            let turma = db::Turma {
                id_turma: if id > 0 { Some(id) } else { None },
                id_escola,
                nome_turma: ui.get_turma_form_nome().to_string(),
                id_curso: ui.get_turma_form_id_curso().to_string().parse().unwrap_or(0),
                ano: ui.get_turma_form_ano().to_string().parse().unwrap_or(2026),
                semestre: ui.get_turma_form_semestre().to_string().parse().unwrap_or(1),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_turma(&conn_save.borrow(), id, &turma)
            } else {
                db::inserir_turma(&conn_save.borrow(), &turma).map(|_| ())
            };

            match res {
                Ok(_) => {
                    let operacao = if modo == "editar" { "atualizada" } else { "criada" };
                    println!("✅ Turma {} com sucesso (ID: {})", operacao, id);
                    ui.set_turma_modo("listar".into());
                    carregar(&ui, &conn_save.borrow(), id_escola);
                }
                Err(e) => {
                    eprintln!("❌ Erro ao salvar turma: {:?}", e);
                }
            }
        }
    });

    // ==========================================
    // 2. EDITAR (CARREGAR FORMULÁRIO)
    // ==========================================
    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_turma_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            // Busca direta por ID - Performance otimizada
            match db::buscar_turma_por_id(&conn_edit.borrow(), id) {
                Ok(turma) => {
                    println!("📝 Carregando turma para edição (ID: {})", id);
                    
                    ui.set_turma_form_id(id);
                    ui.set_turma_form_nome(turma.nome_turma.into());
                    ui.set_turma_form_id_curso(turma.id_curso.to_string().into());
                    ui.set_turma_form_ano(turma.ano.to_string().into());
                    ui.set_turma_form_semestre(turma.semestre.to_string().into());
                    ui.set_turma_modo("editar".into());
                }
                Err(e) => {
                    eprintln!("❌ Erro ao buscar turma (ID: {}): {:?}", id, e);
                }
            }
        }
    });

    // ==========================================
    // 3. DELETAR
    // ==========================================
    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_turma_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            match db::remover_turma(&conn_del.borrow(), id) {
                Ok(_) => {
                    println!("🗑️  Turma removida (ID: {})", id);
                    carregar(&ui, &conn_del.borrow(), id_escola);
                }
                Err(e) => {
                    eprintln!("❌ Erro ao deletar turma (ID: {}): {:?}", id, e);
                }
            }
        }
    });
}

// ==========================================
// 4. CARREGAR (SINCRONIZAÇÃO BANCO -> UI)
// ==========================================
pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32) {
    match db::listar_turmas(conn, id_escola) {
        Ok(turmas) => {
            let total = turmas.len();
            
            let rows: Vec<crate::TurmaRow> = turmas.into_iter().map(|t| crate::TurmaRow {
                id: t.id_turma, // TurmaInfo já tem i32 direto
                nome: t.nome_turma.into(), 
                id_curso: t.id_curso,
                nome_curso: t.nome_curso.into(),
                ano: t.ano, 
                semestre: t.semestre,
            }).collect();
            
            ui.set_turmas_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
            println!("📊 Turmas carregadas: {} registro(s) | Escola: {}", total, id_escola);
        }
        Err(e) => {
            eprintln!("❌ Erro ao carregar turmas: {:?}", e);
        }
    }
}