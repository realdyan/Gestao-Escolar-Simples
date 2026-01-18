slint::include_modules!();

mod mods;
mod db;
mod filtros_avancados;
mod db_paginacao; // ✅ DECLARAÇÃO OBRIGATÓRIA PARA RESOLVER E0432

use std::rc::Rc;
use std::cell::RefCell;
use slint::{ComponentHandle, ModelRc, VecModel};

fn main() {
    let ui = MainWindow::new().unwrap();
    let conn = Rc::new(RefCell::new(db::inicializar_db().expect("❌ Erro ao inicializar banco")));
    
    // 1. CARREGAR UNIDADES PARA O HEADERBAR
    let escolas_rows = if let Ok(escolas) = db::listar_escolas(&conn.borrow()) {
        escolas.into_iter().map(|e| crate::EscolaRow {
            id: e.id_escola.unwrap_or(0),
            nome: e.nome_escola.into(),
            email: e.email.into(), telefone: e.telefone.into(),
            endereco: e.endereco.into(), website: e.website.into(),
        }).collect::<Vec<_>>()
    } else { Vec::new() };

    ui.set_lista_escolas_selecao(ModelRc::from(Rc::new(VecModel::from(escolas_rows))));

    // Configuração de Contexto Inicial
    ui.set_id_escola_ativa(1);
    ui.set_nome_escola_ativa("Escola Padrão".into());

    // 2. CALLBACK DE TROCA DE UNIDADE NO HEADER
    let ui_weak = ui.as_weak();
    ui.on_selecionar_escola(move |id, nome| {
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_id_escola_ativa(id);
            ui.set_nome_escola_ativa(nome.clone());
            println!("🏫 ERP Contexto alterado para: {}", nome);
        }
    });

    ui.on_fechar_app(|| { std::process::exit(0); });

    // Inicialização de todos os módulos de lógica (Sem cortes)
    mods::alunos::setup(&ui, conn.clone());
    mods::escolas::setup(&ui, conn.clone());
    mods::professores::setup(&ui, conn.clone());
    mods::turmas::setup(&ui, conn.clone());
    mods::cursos::setup(&ui, conn.clone());
    mods::disciplinas::setup(&ui, conn.clone());
    mods::matriculas::setup(&ui, conn.clone());

    // 3. TIMER DE SINCRONIZAÇÃO INTELIGENTE (CORRIGIDO)
    let timer = slint::Timer::default();
    {
        let ui_weak = ui.as_weak();
        let conn_timer = conn.clone();
        let mut ultima_tela = String::new();
        let mut ultima_entidade = String::new();
        let mut ultimo_id_escola = 0;
        
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
            if let Some(ui) = ui_weak.upgrade() {
                let tela_atual = ui.get_tela_ativa().to_string();
                let entidade_atual = ui.get_entidade_ativa().to_string();
                let id_escola = ui.get_id_escola_ativa();

                if tela_atual != ultima_tela || entidade_atual != ultima_entidade || id_escola != ultimo_id_escola {
                    match tela_atual.as_str() {
                        "cadastros" => {
                            match entidade_atual.as_str() {
                                // ✅ CORREÇÃO: Passando os 5 argumentos (ui, conn, id, pagina, filtro)
                                "alunos" => mods::alunos::carregar(&ui, &conn_timer.borrow(), id_escola, 1, ""),
                                "professores" => mods::professores::carregar(&ui, &conn_timer.borrow(), id_escola),
                                "escolas" => mods::escolas::carregar(&ui, &conn_timer.borrow()),
                                "turmas" => mods::turmas::carregar(&ui, &conn_timer.borrow(), id_escola),
                                "cursos" => mods::cursos::carregar(&ui, &conn_timer.borrow(), id_escola),
                                "disciplinas" => mods::disciplinas::carregar(&ui, &conn_timer.borrow(), id_escola),
                                _ => {}
                            }
                        },
                        "matriculas" => mods::matriculas::carregar(&ui, &conn_timer.borrow(), id_escola),
                        _ => {}
                    }
                    ultima_tela = tela_atual; ultima_entidade = entidade_atual; ultimo_id_escola = id_escola;
                }
            }
        });
    }

    println!("🚀 ERP Escola Modular Profissional Iniciado!");
    ui.run().unwrap();
}