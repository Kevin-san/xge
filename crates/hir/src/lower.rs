
use crate::hir::*;
use parser::ast::{self, Module, Item};
use anyhow::Result;

pub struct LoweringContext {
    symbol_table: SymbolTable,
    next_symbol_id: usize,
    next_block_id: usize,
}

impl LoweringContext {
    pub fn new() -&gt; Self {
        LoweringContext {
            symbol_table: SymbolTable {
                symbols: HashMap::new(),
            },
            next_symbol_id: 0,
            next_block_id: 0,
        }
    }

    pub fn fresh_symbol(&amp;mut self, name: &amp;str) -&gt; Symbol {
        let id = self.next_symbol_id;
        self.next_symbol_id += 1;
        Symbol(format!("{}_{}", name, id))
    }

    pub fn fresh_block(&amp;mut self) -&gt; BasicBlockId {
        let id = self.next_block_id;
        self.next_block_id += 1;
        BasicBlockId(id)
    }
}

pub fn lower_ast_to_hir(ast: Module) -&gt; Result&lt;HirModule&gt; {
    let mut ctx = LoweringContext::new();
    let mut items = Vec::new();

    for item in ast.items {
        match item {
            Item::FnDef(fn_def) =&gt; {
                let hir_fn = lower_function(&amp;mut ctx, fn_def)?;
                items.push(HirItem::Function(hir_fn));
            }
            Item::ClassDef(class_def) =&gt; {
                let hir_struct = lower_class(&amp;mut ctx, class_def)?;
                items.push(HirItem::Struct(hir_struct));
            }
            Item::LetDecl(let_decl) =&gt; {
            }
            Item::Expr(expr) =&gt; {
            }
        }
    }

    Ok(HirModule {
        items,
        symbols: ctx.symbol_table,
    })
}

fn lower_function(ctx: &amp;mut LoweringContext, fn_def: ast::FnDef) -&gt; Result&lt;HirFunction&gt; {
    let name = Symbol(fn_def.name.0);
    let mut params = Vec::new();
    
    for param in fn_def.params {
        let sym = Symbol(param.name.0);
        let ty = lower_type(&amp;param.ty)?;
        params.push(HirParam { name: sym, ty });
    }

    let return_type = if let Some(ty) = fn_def.return_type {
        lower_type(&amp;ty)?
    } else {
        HirType::Primitive(ast::PrimitiveType::I32)
    };

    let sig = HirSignature { params, return_type };
    
    let entry_block = ctx.fresh_block();
    let body = HirBody {
        locals: Vec::new(),
        blocks: vec![HirBasicBlock {
            id: entry_block,
            stmts: Vec::new(),
            terminator: HirTerminator::Return(None),
        }],
    };

    Ok(HirFunction {
        name,
        sig,
        body,
        is_async: fn_def.is_async,
    })
}

fn lower_class(ctx: &amp;mut LoweringContext, class_def: ast::ClassDef) -&gt; Result&lt;HirStruct&gt; {
    let name = Symbol(class_def.name.0);
    let mut fields = Vec::new();

    for field in class_def.fields {
        let sym = Symbol(field.name.0);
        let ty = lower_type(&amp;field.ty)?;
        fields.push(HirField { name: sym, ty });
    }

    Ok(HirStruct { name, fields })
}

fn lower_type(ty: &amp;ast::Type) -&gt; Result&lt;HirType&gt; {
    match ty {
        ast::Type::Primitive(p) =&gt; Ok(HirType::Primitive(p.clone())),
        ast::Type::Named(ident) =&gt; Ok(HirType::Named(Symbol(ident.0.clone()))),
        ast::Type::Generic(base, args) =&gt; {
            let base = lower_type(base)?;
            let mut hir_args = Vec::new();
            for arg in args {
                hir_args.push(lower_type(arg)?);
            }
            Ok(base)
        }
        ast::Type::Function(params, ret) =&gt; {
            let mut hir_params = Vec::new();
            for param in params {
                hir_params.push(lower_type(param)?);
            }
            let hir_ret = lower_type(ret)?;
            Ok(HirType::Function(hir_params, Box::new(hir_ret)))
        }
        ast::Type::Option(inner) =&gt; {
            let inner = lower_type(inner)?;
            Ok(HirType::Option(Box::new(inner)))
        }
        ast::Type::Result(ok, err) =&gt; {
            let ok = lower_type(ok)?;
            let err = lower_type(err)?;
            Ok(HirType::Result(Box::new(ok), Box::new(err)))
        }
    }
}

