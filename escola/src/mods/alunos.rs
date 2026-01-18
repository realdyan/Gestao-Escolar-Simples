use slint::{ComponentHandle, ModelRc, VecModel};
use crate::db;
use crate::db_paginacao;
use crate::filtros_avancados;
use std::rc::Rc;
use std::cell::RefCell;
use rusqlite::Connection;

const REGISTROS_POR_PAGINA: i32 = 50;

pub fn setup(ui: &crate::MainWindow, conn: Rc<RefCell<Connection>>) {
    let ui_weak = ui.as_weak();
    let conn_save = conn.clone();

    // ==========================================
    // 1. SALVAR (NOVO OU EDIÇÃO)
    // ==========================================
    ui.on_aluno_salvar_cadastro(move |n, nc, e, t, en, c, dn, dm| {
        if let Some(ui) = ui_weak.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let id = ui.get_aluno_form_id();
            let modo = ui.get_aluno_modo().to_string();

            let aluno = db::Aluno {
                id_aluno: if id > 0 { Some(id) } else { None },
                id_escola,
                nome_aluno: n.to_string(),
                nome_completo: nc.to_string(),
                email: e.to_string(),
                telefone: t.to_string(),
                endereco: en.to_string(),
                cpf: c.to_string(),
                data_nascimento: dn.to_string(),
                data_matricula: dm.to_string(),
            };

            let res = if id > 0 && modo == "editar" {
                db::atualizar_aluno(&conn_save.borrow(), id, &aluno)
            } else {
                db::inserir_aluno(&conn_save.borrow(), &aluno).map(|_| ())
            };

            match res {
                Ok(_) => {
                    let operacao = if modo == "editar" { "atualizado" } else { "cadastrado" };
                    println!("✅ Aluno {} com sucesso (ID: {})", operacao, id);
                    
                    ui.set_aluno_modo("listar".into());
                    
                    // Recarrega com filtros atuais
                    let pagina = ui.get_aluno_pagina_atual();
                    let filtro = ui.get_aluno_filtro().to_string();
                    carregar(&ui, &conn_save.borrow(), id_escola, pagina, &filtro);
                }
                Err(e) => {
                    eprintln!("❌ Erro ao salvar aluno: {:?}", e);
                }
            }
        }
    });

    // ==========================================
    // 2. EDITAR
    // ==========================================
    let ui_edit = ui.as_weak();
    let conn_edit = conn.clone();
    ui.on_aluno_editar(move |id| {
        if let Some(ui) = ui_edit.upgrade() {
            match db::buscar_aluno_por_id(&conn_edit.borrow(), id) {
                Ok(aluno) => {
                    println!("📝 Carregando aluno para edição (ID: {})", id);
                    
                    ui.set_aluno_form_id(id);
                    ui.set_aluno_form_nome(aluno.nome_aluno.into());
                    ui.set_aluno_form_nome_completo(aluno.nome_completo.into());
                    ui.set_aluno_form_email(aluno.email.into());
                    ui.set_aluno_form_telefone(aluno.telefone.into());
                    ui.set_aluno_form_endereco(aluno.endereco.into());
                    ui.set_aluno_form_cpf(aluno.cpf.into());
                    ui.set_aluno_form_data_nasc(aluno.data_nascimento.into());
                    ui.set_aluno_form_data_matricula(aluno.data_matricula.into());
                    ui.set_aluno_modo("editar".into());
                }
                Err(e) => {
                    eprintln!("❌ Erro ao buscar aluno (ID: {}): {:?}", id, e);
                }
            }
        }
    });

    // ==========================================
    // 3. DELETAR
    // ==========================================
    let ui_del = ui.as_weak();
    let conn_del = conn.clone();
    ui.on_aluno_deletar(move |id| {
        if let Some(ui) = ui_del.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            match db::remover_aluno(&conn_del.borrow(), id) {
                Ok(_) => {
                    println!("🗑️  Aluno removido (ID: {})", id);
                    
                    let pagina = ui.get_aluno_pagina_atual();
                    let filtro = ui.get_aluno_filtro().to_string();
                    carregar(&ui, &conn_del.borrow(), id_escola, pagina, &filtro);
                }
                Err(e) => {
                    eprintln!("❌ Erro ao deletar aluno (ID: {}): {:?}", id, e);
                }
            }
        }
    });

    // ==========================================
    // 4. ATUALIZAR
    // ==========================================
    let ui_atualizar = ui.as_weak();
    let conn_atualizar = conn.clone();
    ui.on_aluno_atualizar(move || {
        if let Some(ui) = ui_atualizar.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let pagina = ui.get_aluno_pagina_atual();
            let filtro = ui.get_aluno_filtro().to_string();
            
            println!("🔄 Atualizando lista de alunos...");
            carregar(&ui, &conn_atualizar.borrow(), id_escola, pagina, &filtro);
        }
    });

    // ==========================================
    // 5. IR PARA PÁGINA
    // ==========================================
    let ui_pag = ui.as_weak();
    let conn_pag = conn.clone();
    ui.on_aluno_ir_pagina(move |pagina| {
        if let Some(ui) = ui_pag.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let filtro = ui.get_aluno_filtro().to_string();
            
            println!("📄 Indo para página {} de alunos", pagina);
            carregar(&ui, &conn_pag.borrow(), id_escola, pagina, &filtro);
        }
    });

    // ==========================================
    // 6. APLICAR FILTRO SIMPLES
    // ==========================================
    let ui_filtro = ui.as_weak();
    let conn_filtro = conn.clone();
    ui.on_aluno_aplicar_filtro(move |termo_filtro| {
        if let Some(ui) = ui_filtro.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            let filtro_str = termo_filtro.to_string();
            
            ui.set_aluno_filtro(termo_filtro);
            
            println!("🔍 Aplicando filtro simples: '{}'", filtro_str);
            carregar(&ui, &conn_filtro.borrow(), id_escola, 1, &filtro_str);
        }
    });

    // ==========================================
    // 7. LIMPAR FILTRO SIMPLES
    // ==========================================
    let ui_limpar = ui.as_weak();
    let conn_limpar = conn.clone();
    ui.on_aluno_limpar_filtro(move || {
        if let Some(ui) = ui_limpar.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            
            ui.set_aluno_filtro("".into());
            
            println!("🧹 Limpando filtro de alunos");
            carregar(&ui, &conn_limpar.borrow(), id_escola, 1, "");
        }
    });

    // ==========================================
    // 8. ✅ CORRIGIDO: APLICAR FILTRO AVANÇADO
    // ==========================================
    let ui_filtro_avancado = ui.as_weak();
    let conn_filtro_avancado = conn.clone();
    ui.on_alunos_aplicar_filtro_avancado(move |nome, cpf, email, telefone, data_inicio, data_fim| {
        if let Some(ui) = ui_filtro_avancado.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            
            println!("🔍 Aplicando filtro avançado de alunos");
            println!("🎯 CALLBACK RECEBIDO!");
        println!("   Nome recebido: '{}'", nome);
        println!("   CPF recebido: '{}'", cpf);
        println!("   Email recebido: '{}'", email);
            
            match filtros_avancados::filtrar_alunos(
                &conn_filtro_avancado.borrow(),
                id_escola,
                1,
                REGISTROS_POR_PAGINA,
                &nome.to_string(),
                &cpf.to_string(),
                &email.to_string(),
                &telefone.to_string(),  // ✅ ADICIONADO
                &data_inicio.to_string(),
                &data_fim.to_string(),
            ) {
                Ok((alunos, total)) => {
                    let total_paginas = if total > 0 {
                        (total + REGISTROS_POR_PAGINA - 1) / REGISTROS_POR_PAGINA
                    } else {
                        1
                    };
                    
                    let qtd_exibindo = alunos.len();
                    
                    let rows: Vec<crate::AlunoRow> = alunos.into_iter().map(|a| crate::AlunoRow {
                        id: a.id_aluno.unwrap_or(0),
                        nome: a.nome_aluno.into(),
                        email: a.email.into(),
                        cpf: a.cpf.into(),
                        telefone: a.telefone.into(),
                        data_matricula: a.data_matricula.into(),
                    }).collect();
                    
                    ui.set_alunos_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
                    ui.set_aluno_pagina_atual(1);
                    ui.set_aluno_total_paginas(total_paginas);
                    ui.set_aluno_total_registros(total);
                    
                    println!("✅ Filtro avançado aplicado | Exibindo {} de {} alunos", qtd_exibindo, total);
                }
                Err(e) => {
                    eprintln!("❌ Erro ao aplicar filtro avançado: {:?}", e);
                }
            }
        }
    });

    // ==========================================
    // 9. ✅ CORRIGIDO: LIMPAR FILTRO AVANÇADO
    // ==========================================
    let ui_limpar_avancado = ui.as_weak();
    let conn_limpar_avancado = conn.clone();
    ui.on_alunos_limpar_filtro_avancado(move || {
        if let Some(ui) = ui_limpar_avancado.upgrade() {
            let id_escola = ui.get_id_escola_ativa();
            
            // ✅ CORRIGIDO: Use os nomes CORRETOS das propriedades
            ui.set_alunos_filtro_nome("".into());
            ui.set_alunos_filtro_cpf("".into());
            ui.set_alunos_filtro_email("".into());
            ui.set_alunos_filtro_telefone("".into());
            ui.set_alunos_filtro_data_inicio("".into());
            ui.set_alunos_filtro_data_fim("".into());
            
            println!("🧹 Limpando filtro avançado de alunos");
            carregar(&ui, &conn_limpar_avancado.borrow(), id_escola, 1, "");
        }
    });
}

// ==========================================
// 10. CARREGAR DADOS
// ==========================================
pub fn carregar(ui: &crate::MainWindow, conn: &Connection, id_escola: i32, pagina: i32, filtro: &str) {
    match db_paginacao::listar_alunos_paginado(conn, id_escola, pagina, REGISTROS_POR_PAGINA, filtro) {
        Ok((alunos, total)) => {
            let total_paginas = if total > 0 {
                (total + REGISTROS_POR_PAGINA - 1) / REGISTROS_POR_PAGINA
            } else {
                1
            };
            
            let qtd_exibindo = alunos.len();
            
            let rows: Vec<crate::AlunoRow> = alunos.into_iter().map(|a| crate::AlunoRow {
                id: a.id_aluno.unwrap_or(0),
                nome: a.nome_aluno.into(),
                email: a.email.into(),
                cpf: a.cpf.into(),
                telefone: a.telefone.into(),
                data_matricula: a.data_matricula.into(),
            }).collect();
            
            ui.set_alunos_dados(ModelRc::from(Rc::new(VecModel::from(rows))));
            ui.set_aluno_pagina_atual(pagina);
            ui.set_aluno_total_paginas(total_paginas);
            ui.set_aluno_total_registros(total);
            
            if filtro.is_empty() {
                println!("📊 Alunos | Página {}/{} | Exibindo {} de {} | Escola: {}", 
                         pagina, total_paginas, qtd_exibindo, total, id_escola);
            } else {
                println!("🔍 Alunos filtrados | Página {}/{} | Exibindo {} de {} | Filtro: '{}' | Escola: {}", 
                         pagina, total_paginas, qtd_exibindo, total, filtro, id_escola);
            }
        }
        Err(e) => {
            eprintln!("❌ Erro ao carregar alunos: {:?}", e);
        }
    }
}