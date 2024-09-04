// Generated from grammar/CycloneParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::cycloneparserlistener::*;
use super::cycloneparservisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const ARROW:isize=1; 
		pub const BI_ARROW:isize=2; 
		pub const AT_SIGN:isize=3; 
		pub const BAR:isize=4; 
		pub const COLON:isize=5; 
		pub const COLON_COLON:isize=6; 
		pub const COLON_EQUAL:isize=7; 
		pub const COMMA:isize=8; 
		pub const DOT:isize=9; 
		pub const DOTDOT:isize=10; 
		pub const EQUAL:isize=11; 
		pub const GREATER:isize=12; 
		pub const GREATER_EQUAL:isize=13; 
		pub const HASH:isize=14; 
		pub const LBRACE:isize=15; 
		pub const LBRACK:isize=16; 
		pub const LESS:isize=17; 
		pub const LESS_EQUAL:isize=18; 
		pub const LPAREN:isize=19; 
		pub const MINUS:isize=20; 
		pub const NOT_EQUAL:isize=21; 
		pub const RBRACE:isize=22; 
		pub const RBRACK:isize=23; 
		pub const RPAREN:isize=24; 
		pub const SEMI:isize=25; 
		pub const SLASH:isize=26; 
		pub const STAR:isize=27; 
		pub const PLUS:isize=28; 
		pub const XOR:isize=29; 
		pub const IMPLIES:isize=30; 
		pub const NOT:isize=31; 
		pub const HAT:isize=32; 
		pub const P_OP_ONE:isize=33; 
		pub const BIT_AND:isize=34; 
		pub const BIT_NEGATION:isize=35; 
		pub const PLUS_PLUS:isize=36; 
		pub const MINUS_MINUS:isize=37; 
		pub const TIMES_TIMES:isize=38; 
		pub const MOD:isize=39; 
		pub const OR:isize=40; 
		pub const AND:isize=41; 
		pub const ASSIGN_PLUS_EQ:isize=42; 
		pub const ASSIGN_MINUS_EQ:isize=43; 
		pub const ASSIGN_TIMES_EQ:isize=44; 
		pub const ASSIGN_DIV_EQ:isize=45; 
		pub const ASSIGN_SHIFT_LEFT:isize=46; 
		pub const ASSIGN_SHIFT_RIGHT:isize=47; 
		pub const SHIFT_LEFT:isize=48; 
		pub const SHIFT_RIGHT:isize=49; 
		pub const DOUBLE_EQUAL:isize=50; 
		pub const GLOBAL:isize=51; 
		pub const NATIVE:isize=52; 
		pub const DEBUG:isize=53; 
		pub const LOG:isize=54; 
		pub const OUTPUT:isize=55; 
		pub const TRACE:isize=56; 
		pub const PRECISION:isize=57; 
		pub const TIMEOUT:isize=58; 
		pub const DETECT:isize=59; 
		pub const BVDISPLAY:isize=60; 
		pub const STATE:isize=61; 
		pub const NODE:isize=62; 
		pub const MACHINE:isize=63; 
		pub const GRAPH:isize=64; 
		pub const TRANS1:isize=65; 
		pub const TRANS2:isize=66; 
		pub const EDGE:isize=67; 
		pub const RECORD:isize=68; 
		pub const CONST:isize=69; 
		pub const ON:isize=70; 
		pub const LABEL:isize=71; 
		pub const INVARIANT:isize=72; 
		pub const INT:isize=73; 
		pub const BOOL:isize=74; 
		pub const REAL:isize=75; 
		pub const CHAR:isize=76; 
		pub const STRING:isize=77; 
		pub const ENUM:isize=78; 
		pub const WHERE:isize=79; 
		pub const START:isize=80; 
		pub const FINAL:isize=81; 
		pub const ABSTRACT:isize=82; 
		pub const NORMAL:isize=83; 
		pub const PREV:isize=84; 
		pub const GOAL:isize=85; 
		pub const CHECK:isize=86; 
		pub const FOR:isize=87; 
		pub const STOP:isize=88; 
		pub const AT:isize=89; 
		pub const VIA:isize=90; 
		pub const CONDITION:isize=91; 
		pub const REACH:isize=92; 
		pub const WITH:isize=93; 
		pub const ENUMERATE:isize=94; 
		pub const LET:isize=95; 
		pub const EACH:isize=96; 
		pub const ASSERT:isize=97; 
		pub const INITIAL:isize=98; 
		pub const IN:isize=99; 
		pub const FRESH:isize=100; 
		pub const OPTION:isize=101; 
		pub const ALWAYS:isize=102; 
		pub const SOME:isize=103; 
		pub const ONE:isize=104; 
		pub const UPTO:isize=105; 
		pub const FUNCTION:isize=106; 
		pub const RETURN:isize=107; 
		pub const IF:isize=108; 
		pub const ELSE:isize=109; 
		pub const BV:isize=110; 
		pub const INTLITERAL:isize=111; 
		pub const BVLITERAL:isize=112; 
		pub const REALLITERAL:isize=113; 
		pub const CHARLITERAL:isize=114; 
		pub const STRINGLITERAL:isize=115; 
		pub const BOOLLITERAL:isize=116; 
		pub const ENUMLITERAL:isize=117; 
		pub const IDENT:isize=118; 
		pub const ML_COMMENT:isize=119; 
		pub const SL_COMMENT:isize=120; 
		pub const WS:isize=121;
	pub const RULE_identifier:usize = 0; 
	pub const RULE_compOptions:usize = 1; 
	pub const RULE_optionName:usize = 2; 
	pub const RULE_statementList:usize = 3; 
	pub const RULE_transList:usize = 4; 
	pub const RULE_letOrPathAssignExpr:usize = 5; 
	pub const RULE_globalDefinitions:usize = 6; 
	pub const RULE_program:usize = 7; 
	pub const RULE_machineDecl:usize = 8; 
	pub const RULE_machineScope:usize = 9; 
	pub const RULE_stateExpr:usize = 10; 
	pub const RULE_stateScope:usize = 11; 
	pub const RULE_trans:usize = 12; 
	pub const RULE_transScope:usize = 13; 
	pub const RULE_transOp:usize = 14; 
	pub const RULE_transDef:usize = 15; 
	pub const RULE_transExclExpr:usize = 16; 
	pub const RULE_invariantExpression:usize = 17; 
	pub const RULE_inExpr:usize = 18; 
	pub const RULE_invariantScope:usize = 19; 
	pub const RULE_goal:usize = 20; 
	pub const RULE_checkExpr:usize = 21; 
	pub const RULE_forExpr:usize = 22; 
	pub const RULE_stopExpr:usize = 23; 
	pub const RULE_viaExpr:usize = 24; 
	pub const RULE_pathExprList:usize = 25; 
	pub const RULE_withExpr:usize = 26; 
	pub const RULE_letExpr:usize = 27; 
	pub const RULE_pathAssignStatement:usize = 28; 
	pub const RULE_pathCondAssignExpr:usize = 29; 
	pub const RULE_pathExpr:usize = 30; 
	pub const RULE_pathCondition:usize = 31; 
	pub const RULE_orPathCondition:usize = 32; 
	pub const RULE_andPathCondition:usize = 33; 
	pub const RULE_xorPathCondition:usize = 34; 
	pub const RULE_unaryPathCondition:usize = 35; 
	pub const RULE_primaryCondition:usize = 36; 
	pub const RULE_parPathCondition:usize = 37; 
	pub const RULE_stateIncExpr:usize = 38; 
	pub const RULE_pathPrimaryExpr:usize = 39; 
	pub const RULE_pathOp:usize = 40; 
	pub const RULE_label:usize = 41; 
	pub const RULE_stateModifier:usize = 42; 
	pub const RULE_literal:usize = 43; 
	pub const RULE_intLiteral:usize = 44; 
	pub const RULE_realLiteral:usize = 45; 
	pub const RULE_boolLiteral:usize = 46; 
	pub const RULE_stringLiteral:usize = 47; 
	pub const RULE_charLiteral:usize = 48; 
	pub const RULE_bvLiteral:usize = 49; 
	pub const RULE_enumLiteral:usize = 50; 
	pub const RULE_record:usize = 51; 
	pub const RULE_recordScope:usize = 52; 
	pub const RULE_recordVariableDeclGroup:usize = 53; 
	pub const RULE_recordVariableDecl:usize = 54; 
	pub const RULE_globalConstantGroup:usize = 55; 
	pub const RULE_globalConstantDecl:usize = 56; 
	pub const RULE_globalVariableGroup:usize = 57; 
	pub const RULE_localVariableGroup:usize = 58; 
	pub const RULE_modifier:usize = 59; 
	pub const RULE_type_mark:usize = 60; 
	pub const RULE_primitiveBvType:usize = 61; 
	pub const RULE_bvType:usize = 62; 
	pub const RULE_primitiveType:usize = 63; 
	pub const RULE_enumType:usize = 64; 
	pub const RULE_enumDecl:usize = 65; 
	pub const RULE_variableDeclarator:usize = 66; 
	pub const RULE_whereExpr:usize = 67; 
	pub const RULE_variableInitializer:usize = 68; 
	pub const RULE_assertExpr:usize = 69; 
	pub const RULE_assertMainExpr:usize = 70; 
	pub const RULE_statement:usize = 71; 
	pub const RULE_expression:usize = 72; 
	pub const RULE_conditionalImpliesExpression:usize = 73; 
	pub const RULE_conditionalOrExpression:usize = 74; 
	pub const RULE_conditionalAndExpression:usize = 75; 
	pub const RULE_conditionalXorExpression:usize = 76; 
	pub const RULE_bitwiseOrExpression:usize = 77; 
	pub const RULE_bitwiseAndExpression:usize = 78; 
	pub const RULE_equalityExpression:usize = 79; 
	pub const RULE_relationalExpression:usize = 80; 
	pub const RULE_bitShiftExpression:usize = 81; 
	pub const RULE_additiveExpression:usize = 82; 
	pub const RULE_multiplicativeExpression:usize = 83; 
	pub const RULE_powExpression:usize = 84; 
	pub const RULE_unaryExpression:usize = 85; 
	pub const RULE_unaryExpressionNotPlusMinus:usize = 86; 
	pub const RULE_oneExpr:usize = 87; 
	pub const RULE_freshExpr:usize = 88; 
	pub const RULE_initialExpr:usize = 89; 
	pub const RULE_prevExpr:usize = 90; 
	pub const RULE_functionDeclaration:usize = 91; 
	pub const RULE_functionBodyScope:usize = 92; 
	pub const RULE_functionParamsDecl:usize = 93; 
	pub const RULE_functionParam:usize = 94; 
	pub const RULE_returnExpr:usize = 95; 
	pub const RULE_primary:usize = 96; 
	pub const RULE_dotIdentifierExpr:usize = 97; 
	pub const RULE_parExpression:usize = 98; 
	pub const RULE_funCall:usize = 99; 
	pub const RULE_iteStatement:usize = 100; 
	pub const RULE_annotationExpr:usize = 101;
	pub const ruleNames: [&'static str; 102] =  [
		"identifier", "compOptions", "optionName", "statementList", "transList", 
		"letOrPathAssignExpr", "globalDefinitions", "program", "machineDecl", 
		"machineScope", "stateExpr", "stateScope", "trans", "transScope", "transOp", 
		"transDef", "transExclExpr", "invariantExpression", "inExpr", "invariantScope", 
		"goal", "checkExpr", "forExpr", "stopExpr", "viaExpr", "pathExprList", 
		"withExpr", "letExpr", "pathAssignStatement", "pathCondAssignExpr", "pathExpr", 
		"pathCondition", "orPathCondition", "andPathCondition", "xorPathCondition", 
		"unaryPathCondition", "primaryCondition", "parPathCondition", "stateIncExpr", 
		"pathPrimaryExpr", "pathOp", "label", "stateModifier", "literal", "intLiteral", 
		"realLiteral", "boolLiteral", "stringLiteral", "charLiteral", "bvLiteral", 
		"enumLiteral", "record", "recordScope", "recordVariableDeclGroup", "recordVariableDecl", 
		"globalConstantGroup", "globalConstantDecl", "globalVariableGroup", "localVariableGroup", 
		"modifier", "type_mark", "primitiveBvType", "bvType", "primitiveType", 
		"enumType", "enumDecl", "variableDeclarator", "whereExpr", "variableInitializer", 
		"assertExpr", "assertMainExpr", "statement", "expression", "conditionalImpliesExpression", 
		"conditionalOrExpression", "conditionalAndExpression", "conditionalXorExpression", 
		"bitwiseOrExpression", "bitwiseAndExpression", "equalityExpression", "relationalExpression", 
		"bitShiftExpression", "additiveExpression", "multiplicativeExpression", 
		"powExpression", "unaryExpression", "unaryExpressionNotPlusMinus", "oneExpr", 
		"freshExpr", "initialExpr", "prevExpr", "functionDeclaration", "functionBodyScope", 
		"functionParamsDecl", "functionParam", "returnExpr", "primary", "dotIdentifierExpr", 
		"parExpression", "funCall", "iteStatement", "annotationExpr"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;111] = [
		None, Some("'->'"), Some("'<->'"), Some("'@'"), Some("'|'"), Some("':'"), 
		Some("'::'"), Some("':='"), Some("','"), Some("'.'"), Some("'..'"), Some("'='"), 
		Some("'>'"), Some("'>='"), Some("'#'"), Some("'{'"), Some("'['"), Some("'<'"), 
		Some("'<='"), Some("'('"), Some("'-'"), Some("'!='"), Some("'}'"), Some("']'"), 
		Some("')'"), Some("';'"), Some("'/'"), Some("'*'"), Some("'+'"), Some("'xor'"), 
		Some("'=>'"), Some("'!'"), Some("'^'"), Some("'_'"), Some("'&'"), Some("'~'"), 
		Some("'++'"), Some("'--'"), Some("'**'"), Some("'%'"), Some("'||'"), Some("'&&'"), 
		Some("'+='"), Some("'-='"), Some("'*='"), Some("'/='"), Some("'<<='"), 
		Some("'>>='"), Some("'<<'"), Some("'>>'"), Some("'=='"), Some("'global'"), 
		Some("'native'"), Some("'debug'"), Some("'log'"), Some("'output'"), Some("'trace'"), 
		Some("'precision'"), Some("'timeout'"), Some("'detect'"), Some("'bvdisplay'"), 
		Some("'state'"), Some("'node'"), Some("'machine'"), Some("'graph'"), Some("'transition'"), 
		Some("'trans'"), Some("'edge'"), Some("'record'"), Some("'const'"), Some("'on'"), 
		Some("'label'"), Some("'invariant'"), Some("'int'"), Some("'bool'"), Some("'real'"), 
		Some("'char'"), Some("'string'"), Some("'enum'"), Some("'where'"), Some("'start'"), 
		Some("'final'"), Some("'abstract'"), Some("'normal'"), Some("'prev'"), 
		Some("'goal'"), Some("'check'"), Some("'for'"), Some("'stop'"), Some("'at'"), 
		Some("'via'"), Some("'condition'"), Some("'reach'"), Some("'with'"), Some("'enumerate'"), 
		Some("'let'"), Some("'each'"), Some("'assert'"), Some("'initial'"), Some("'in'"), 
		Some("'fresh'"), Some("'option-'"), Some("'always'"), Some("'some'"), 
		Some("'one'"), Some("'upto'"), Some("'function'"), Some("'return'"), Some("'if'"), 
		Some("'else'"), Some("'bv'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;122]  = [
		None, Some("ARROW"), Some("BI_ARROW"), Some("AT_SIGN"), Some("BAR"), Some("COLON"), 
		Some("COLON_COLON"), Some("COLON_EQUAL"), Some("COMMA"), Some("DOT"), 
		Some("DOTDOT"), Some("EQUAL"), Some("GREATER"), Some("GREATER_EQUAL"), 
		Some("HASH"), Some("LBRACE"), Some("LBRACK"), Some("LESS"), Some("LESS_EQUAL"), 
		Some("LPAREN"), Some("MINUS"), Some("NOT_EQUAL"), Some("RBRACE"), Some("RBRACK"), 
		Some("RPAREN"), Some("SEMI"), Some("SLASH"), Some("STAR"), Some("PLUS"), 
		Some("XOR"), Some("IMPLIES"), Some("NOT"), Some("HAT"), Some("P_OP_ONE"), 
		Some("BIT_AND"), Some("BIT_NEGATION"), Some("PLUS_PLUS"), Some("MINUS_MINUS"), 
		Some("TIMES_TIMES"), Some("MOD"), Some("OR"), Some("AND"), Some("ASSIGN_PLUS_EQ"), 
		Some("ASSIGN_MINUS_EQ"), Some("ASSIGN_TIMES_EQ"), Some("ASSIGN_DIV_EQ"), 
		Some("ASSIGN_SHIFT_LEFT"), Some("ASSIGN_SHIFT_RIGHT"), Some("SHIFT_LEFT"), 
		Some("SHIFT_RIGHT"), Some("DOUBLE_EQUAL"), Some("GLOBAL"), Some("NATIVE"), 
		Some("DEBUG"), Some("LOG"), Some("OUTPUT"), Some("TRACE"), Some("PRECISION"), 
		Some("TIMEOUT"), Some("DETECT"), Some("BVDISPLAY"), Some("STATE"), Some("NODE"), 
		Some("MACHINE"), Some("GRAPH"), Some("TRANS1"), Some("TRANS2"), Some("EDGE"), 
		Some("RECORD"), Some("CONST"), Some("ON"), Some("LABEL"), Some("INVARIANT"), 
		Some("INT"), Some("BOOL"), Some("REAL"), Some("CHAR"), Some("STRING"), 
		Some("ENUM"), Some("WHERE"), Some("START"), Some("FINAL"), Some("ABSTRACT"), 
		Some("NORMAL"), Some("PREV"), Some("GOAL"), Some("CHECK"), Some("FOR"), 
		Some("STOP"), Some("AT"), Some("VIA"), Some("CONDITION"), Some("REACH"), 
		Some("WITH"), Some("ENUMERATE"), Some("LET"), Some("EACH"), Some("ASSERT"), 
		Some("INITIAL"), Some("IN"), Some("FRESH"), Some("OPTION"), Some("ALWAYS"), 
		Some("SOME"), Some("ONE"), Some("UPTO"), Some("FUNCTION"), Some("RETURN"), 
		Some("IF"), Some("ELSE"), Some("BV"), Some("INTLITERAL"), Some("BVLITERAL"), 
		Some("REALLITERAL"), Some("CHARLITERAL"), Some("STRINGLITERAL"), Some("BOOLLITERAL"), 
		Some("ENUMLITERAL"), Some("IDENT"), Some("ML_COMMENT"), Some("SL_COMMENT"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,CycloneParserExt<'input>, I, CycloneParserContextType , dyn CycloneParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type CycloneParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, CycloneParserContextType , dyn CycloneParserListener<'input> + 'a>;

/// Parser for CycloneParser grammar
pub struct CycloneParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				CycloneParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> CycloneParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> CycloneParser<'input, I, DefaultErrorStrategy<'input,CycloneParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for CycloneParser
pub trait CycloneParserContext<'input>:
	for<'x> Listenable<dyn CycloneParserListener<'input> + 'x > + 
	for<'x> Visitable<dyn CycloneParserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=CycloneParserContextType>
{}

antlr_rust::coerce_from!{ 'input : CycloneParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn CycloneParserContext<'input> + 'input
where
    T: CycloneParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn CycloneParserVisitor<'input> + 'x))
    }
}

impl<'input> CycloneParserContext<'input> for TerminalNode<'input,CycloneParserContextType> {}
impl<'input> CycloneParserContext<'input> for ErrorNode<'input,CycloneParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn CycloneParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn CycloneParserListener<'input> + 'input }

pub struct CycloneParserContextType;
antlr_rust::tid!{CycloneParserContextType}

impl<'input> ParserNodeType<'input> for CycloneParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn CycloneParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CycloneParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> CycloneParserExt<'input>{
}
antlr_rust::tid! { CycloneParserExt<'a> }

impl<'input> TokenAware<'input> for CycloneParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for CycloneParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for CycloneParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "CycloneParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;


pub type IdentifierContext<'input> = BaseParserRuleContext<'input,IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for IdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for IdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifier(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_identifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for IdentifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_identifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr_rust::tid!{IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}

}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifier(&mut self,)
	-> Result<Rc<IdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(204);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compOptions ----------------
pub type CompOptionsContextAll<'input> = CompOptionsContext<'input>;


pub type CompOptionsContext<'input> = BaseParserRuleContext<'input,CompOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct CompOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for CompOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for CompOptionsContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compOptions(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_compOptions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for CompOptionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_compOptions(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compOptions }
}
antlr_rust::tid!{CompOptionsContextExt<'a>}

impl<'input> CompOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompOptionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompOptionsContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<CompOptionsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPTION
/// Returns `None` if there is no child corresponding to token OPTION
fn OPTION(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(OPTION, 0)
}
fn optionName(&self) -> Option<Rc<OptionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> CompOptionsContextAttrs<'input> for CompOptionsContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compOptions(&mut self,)
	-> Result<Rc<CompOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_compOptions);
        let mut _localctx: Rc<CompOptionsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(206);
			recog.base.match_token(OPTION,&mut recog.err_handler)?;

			/*InvokeRule optionName*/
			recog.base.set_state(207);
			recog.optionName()?;

			recog.base.set_state(208);
			recog.base.match_token(EQUAL,&mut recog.err_handler)?;

			/*InvokeRule literal*/
			recog.base.set_state(209);
			recog.literal()?;

			recog.base.set_state(210);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- optionName ----------------
pub type OptionNameContextAll<'input> = OptionNameContext<'input>;


pub type OptionNameContext<'input> = BaseParserRuleContext<'input,OptionNameContextExt<'input>>;

#[derive(Clone)]
pub struct OptionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for OptionNameContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for OptionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_optionName(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_optionName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for OptionNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_optionName(self);
	}
}

impl<'input> CustomRuleContext<'input> for OptionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionName }
}
antlr_rust::tid!{OptionNameContextExt<'a>}

impl<'input> OptionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OptionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OptionNameContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<OptionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEBUG
/// Returns `None` if there is no child corresponding to token DEBUG
fn DEBUG(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(DEBUG, 0)
}
/// Retrieves first TerminalNode corresponding to token LOG
/// Returns `None` if there is no child corresponding to token LOG
fn LOG(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LOG, 0)
}
/// Retrieves first TerminalNode corresponding to token OUTPUT
/// Returns `None` if there is no child corresponding to token OUTPUT
fn OUTPUT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(OUTPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token TRACE
/// Returns `None` if there is no child corresponding to token TRACE
fn TRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(TRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token PRECISION
/// Returns `None` if there is no child corresponding to token PRECISION
fn PRECISION(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PRECISION, 0)
}
/// Retrieves first TerminalNode corresponding to token TIMEOUT
/// Returns `None` if there is no child corresponding to token TIMEOUT
fn TIMEOUT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(TIMEOUT, 0)
}
/// Retrieves first TerminalNode corresponding to token DETECT
/// Returns `None` if there is no child corresponding to token DETECT
fn DETECT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(DETECT, 0)
}
/// Retrieves first TerminalNode corresponding to token BVDISPLAY
/// Returns `None` if there is no child corresponding to token BVDISPLAY
fn BVDISPLAY(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BVDISPLAY, 0)
}

}

impl<'input> OptionNameContextAttrs<'input> for OptionNameContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn optionName(&mut self,)
	-> Result<Rc<OptionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_optionName);
        let mut _localctx: Rc<OptionNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(212);
			_la = recog.base.input.la(1);
			if { !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (DEBUG - 53)) | (1usize << (LOG - 53)) | (1usize << (OUTPUT - 53)) | (1usize << (TRACE - 53)) | (1usize << (PRECISION - 53)) | (1usize << (TIMEOUT - 53)) | (1usize << (DETECT - 53)) | (1usize << (BVDISPLAY - 53)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statementList ----------------
pub type StatementListContextAll<'input> = StatementListContext<'input>;


pub type StatementListContext<'input> = BaseParserRuleContext<'input,StatementListContextExt<'input>>;

#[derive(Clone)]
pub struct StatementListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StatementListContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StatementListContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statementList(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_statementList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StatementListContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_statementList(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statementList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statementList }
}
antlr_rust::tid!{StatementListContextExt<'a>}

impl<'input> StatementListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementListContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StatementListContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StatementListContextAttrs<'input> for StatementListContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statementList(&mut self,)
	-> Result<Rc<StatementListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_statementList);
        let mut _localctx: Rc<StatementListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & ((1usize << (LPAREN - 19)) | (1usize << (MINUS - 19)) | (1usize << (PLUS - 19)) | (1usize << (NOT - 19)) | (1usize << (BIT_NEGATION - 19)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (INITIAL - 98)) | (1usize << (FRESH - 98)) | (1usize << (ONE - 98)) | (1usize << (RETURN - 98)) | (1usize << (INTLITERAL - 98)) | (1usize << (BVLITERAL - 98)) | (1usize << (REALLITERAL - 98)) | (1usize << (CHARLITERAL - 98)) | (1usize << (STRINGLITERAL - 98)) | (1usize << (BOOLLITERAL - 98)) | (1usize << (ENUMLITERAL - 98)) | (1usize << (IDENT - 98)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(214);
				recog.statement()?;

				}
				}
				recog.base.set_state(219);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transList ----------------
pub type TransListContextAll<'input> = TransListContext<'input>;


pub type TransListContext<'input> = BaseParserRuleContext<'input,TransListContextExt<'input>>;

#[derive(Clone)]
pub struct TransListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransListContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransListContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transList(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_transList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransListContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_transList(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transList }
}
antlr_rust::tid!{TransListContextExt<'a>}

impl<'input> TransListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransListContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransListContextExt<'input>>{

fn trans_all(&self) ->  Vec<Rc<TransContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn trans(&self, i: usize) -> Option<Rc<TransContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TransListContextAttrs<'input> for TransListContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transList(&mut self,)
	-> Result<Rc<TransListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_transList);
        let mut _localctx: Rc<TransListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(223);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (TRANS1 - 65)) | (1usize << (TRANS2 - 65)) | (1usize << (EDGE - 65)))) != 0) {
				{
				{
				/*InvokeRule trans*/
				recog.base.set_state(220);
				recog.trans()?;

				}
				}
				recog.base.set_state(225);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- letOrPathAssignExpr ----------------
pub type LetOrPathAssignExprContextAll<'input> = LetOrPathAssignExprContext<'input>;


pub type LetOrPathAssignExprContext<'input> = BaseParserRuleContext<'input,LetOrPathAssignExprContextExt<'input>>;

#[derive(Clone)]
pub struct LetOrPathAssignExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for LetOrPathAssignExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for LetOrPathAssignExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_letOrPathAssignExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_letOrPathAssignExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for LetOrPathAssignExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_letOrPathAssignExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for LetOrPathAssignExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_letOrPathAssignExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_letOrPathAssignExpr }
}
antlr_rust::tid!{LetOrPathAssignExprContextExt<'a>}

impl<'input> LetOrPathAssignExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LetOrPathAssignExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LetOrPathAssignExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LetOrPathAssignExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<LetOrPathAssignExprContextExt<'input>>{

fn letExpr(&self) -> Option<Rc<LetExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pathAssignStatement(&self) -> Option<Rc<PathAssignStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LetOrPathAssignExprContextAttrs<'input> for LetOrPathAssignExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn letOrPathAssignExpr(&mut self,)
	-> Result<Rc<LetOrPathAssignExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LetOrPathAssignExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_letOrPathAssignExpr);
        let mut _localctx: Rc<LetOrPathAssignExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(228);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule letExpr*/
					recog.base.set_state(226);
					recog.letExpr()?;

					}
				}

			 IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule pathAssignStatement*/
					recog.base.set_state(227);
					recog.pathAssignStatement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- globalDefinitions ----------------
pub type GlobalDefinitionsContextAll<'input> = GlobalDefinitionsContext<'input>;


pub type GlobalDefinitionsContext<'input> = BaseParserRuleContext<'input,GlobalDefinitionsContextExt<'input>>;

#[derive(Clone)]
pub struct GlobalDefinitionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for GlobalDefinitionsContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for GlobalDefinitionsContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_globalDefinitions(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_globalDefinitions(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for GlobalDefinitionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_globalDefinitions(self);
	}
}

impl<'input> CustomRuleContext<'input> for GlobalDefinitionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_globalDefinitions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_globalDefinitions }
}
antlr_rust::tid!{GlobalDefinitionsContextExt<'a>}

impl<'input> GlobalDefinitionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GlobalDefinitionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GlobalDefinitionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GlobalDefinitionsContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<GlobalDefinitionsContextExt<'input>>{

fn globalVariableGroup(&self) -> Option<Rc<GlobalVariableGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn globalConstantGroup(&self) -> Option<Rc<GlobalConstantGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn record(&self) -> Option<Rc<RecordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionDeclaration(&self) -> Option<Rc<FunctionDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GlobalDefinitionsContextAttrs<'input> for GlobalDefinitionsContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn globalDefinitions(&mut self,)
	-> Result<Rc<GlobalDefinitionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GlobalDefinitionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_globalDefinitions);
        let mut _localctx: Rc<GlobalDefinitionsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(234);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT | BOOL | REAL | STRING | ENUM | BV 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					/*InvokeRule globalVariableGroup*/
					recog.base.set_state(230);
					recog.globalVariableGroup()?;

					}
					}
				}

			 CONST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					{
					/*InvokeRule globalConstantGroup*/
					recog.base.set_state(231);
					recog.globalConstantGroup()?;

					}
					}
				}

			 RECORD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					{
					/*InvokeRule record*/
					recog.base.set_state(232);
					recog.record()?;

					}
					}
				}

			 FUNCTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					{
					/*InvokeRule functionDeclaration*/
					recog.base.set_state(233);
					recog.functionDeclaration()?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn machineDecl(&self) -> Option<Rc<MachineDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn compOptions_all(&self) ->  Vec<Rc<CompOptionsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compOptions(&self, i: usize) -> Option<Rc<CompOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OPTION {
				{
				{
				/*InvokeRule compOptions*/
				recog.base.set_state(236);
				recog.compOptions()?;

				}
				}
				recog.base.set_state(241);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule machineDecl*/
			recog.base.set_state(242);
			recog.machineDecl()?;

			recog.base.set_state(243);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- machineDecl ----------------
pub type MachineDeclContextAll<'input> = MachineDeclContext<'input>;


pub type MachineDeclContext<'input> = BaseParserRuleContext<'input,MachineDeclContextExt<'input>>;

#[derive(Clone)]
pub struct MachineDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for MachineDeclContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for MachineDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_machineDecl(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_machineDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for MachineDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_machineDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for MachineDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_machineDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_machineDecl }
}
antlr_rust::tid!{MachineDeclContextExt<'a>}

impl<'input> MachineDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MachineDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MachineDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MachineDeclContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<MachineDeclContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn machineScope(&self) -> Option<Rc<MachineScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MACHINE
/// Returns `None` if there is no child corresponding to token MACHINE
fn MACHINE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(MACHINE, 0)
}
/// Retrieves first TerminalNode corresponding to token GRAPH
/// Returns `None` if there is no child corresponding to token GRAPH
fn GRAPH(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(GRAPH, 0)
}

}

impl<'input> MachineDeclContextAttrs<'input> for MachineDeclContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn machineDecl(&mut self,)
	-> Result<Rc<MachineDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MachineDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_machineDecl);
        let mut _localctx: Rc<MachineDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(245);
			_la = recog.base.input.la(1);
			if { !(_la==MACHINE || _la==GRAPH) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule identifier*/
			recog.base.set_state(246);
			recog.identifier()?;

			/*InvokeRule machineScope*/
			recog.base.set_state(247);
			recog.machineScope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- machineScope ----------------
pub type MachineScopeContextAll<'input> = MachineScopeContext<'input>;


pub type MachineScopeContext<'input> = BaseParserRuleContext<'input,MachineScopeContextExt<'input>>;

#[derive(Clone)]
pub struct MachineScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for MachineScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for MachineScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_machineScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_machineScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for MachineScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_machineScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for MachineScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_machineScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_machineScope }
}
antlr_rust::tid!{MachineScopeContextExt<'a>}

impl<'input> MachineScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MachineScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MachineScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MachineScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<MachineScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn stateExpr_all(&self) ->  Vec<Rc<StateExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stateExpr(&self, i: usize) -> Option<Rc<StateExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn trans_all(&self) ->  Vec<Rc<TransContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn trans(&self, i: usize) -> Option<Rc<TransContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn invariantExpression_all(&self) ->  Vec<Rc<InvariantExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn invariantExpression(&self, i: usize) -> Option<Rc<InvariantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn goal(&self) -> Option<Rc<GoalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn globalVariableGroup_all(&self) ->  Vec<Rc<GlobalVariableGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn globalVariableGroup(&self, i: usize) -> Option<Rc<GlobalVariableGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn globalConstantGroup_all(&self) ->  Vec<Rc<GlobalConstantGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn globalConstantGroup(&self, i: usize) -> Option<Rc<GlobalConstantGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn record_all(&self) ->  Vec<Rc<RecordContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn record(&self, i: usize) -> Option<Rc<RecordContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn functionDeclaration_all(&self) ->  Vec<Rc<FunctionDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionDeclaration(&self, i: usize) -> Option<Rc<FunctionDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MachineScopeContextAttrs<'input> for MachineScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn machineScope(&mut self,)
	-> Result<Rc<MachineScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MachineScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_machineScope);
        let mut _localctx: Rc<MachineScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(249);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(256);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (RECORD - 68)) | (1usize << (CONST - 68)) | (1usize << (INT - 68)) | (1usize << (BOOL - 68)) | (1usize << (REAL - 68)) | (1usize << (STRING - 68)) | (1usize << (ENUM - 68)))) != 0) || _la==FUNCTION || _la==BV {
				{
				recog.base.set_state(254);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 INT | BOOL | REAL | STRING | ENUM | BV 
					=> {
						{
						{
						/*InvokeRule globalVariableGroup*/
						recog.base.set_state(250);
						recog.globalVariableGroup()?;

						}
						}
					}

				 CONST 
					=> {
						{
						{
						/*InvokeRule globalConstantGroup*/
						recog.base.set_state(251);
						recog.globalConstantGroup()?;

						}
						}
					}

				 RECORD 
					=> {
						{
						{
						/*InvokeRule record*/
						recog.base.set_state(252);
						recog.record()?;

						}
						}
					}

				 FUNCTION 
					=> {
						{
						{
						/*InvokeRule functionDeclaration*/
						recog.base.set_state(253);
						recog.functionDeclaration()?;

						}
						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(258);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & ((1usize << (STATE - 61)) | (1usize << (NODE - 61)) | (1usize << (START - 61)) | (1usize << (FINAL - 61)) | (1usize << (ABSTRACT - 61)) | (1usize << (NORMAL - 61)))) != 0) {
				{
				{
				/*InvokeRule stateExpr*/
				recog.base.set_state(259);
				recog.stateExpr()?;

				}
				}
				recog.base.set_state(264);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (TRANS1 - 65)) | (1usize << (TRANS2 - 65)) | (1usize << (EDGE - 65)))) != 0) {
				{
				{
				/*InvokeRule trans*/
				recog.base.set_state(265);
				recog.trans()?;

				}
				}
				recog.base.set_state(270);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(274);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==INVARIANT {
				{
				{
				/*InvokeRule invariantExpression*/
				recog.base.set_state(271);
				recog.invariantExpression()?;

				}
				}
				recog.base.set_state(276);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(278);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==GOAL {
				{
				/*InvokeRule goal*/
				recog.base.set_state(277);
				recog.goal()?;

				}
			}

			recog.base.set_state(280);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateExpr ----------------
pub type StateExprContextAll<'input> = StateExprContext<'input>;


pub type StateExprContext<'input> = BaseParserRuleContext<'input,StateExprContextExt<'input>>;

#[derive(Clone)]
pub struct StateExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StateExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StateExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stateExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StateExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stateExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateExpr }
}
antlr_rust::tid!{StateExprContextExt<'a>}

impl<'input> StateExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StateExprContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stateScope(&self) -> Option<Rc<StateScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STATE
/// Returns `None` if there is no child corresponding to token STATE
fn STATE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STATE, 0)
}
/// Retrieves first TerminalNode corresponding to token NODE
/// Returns `None` if there is no child corresponding to token NODE
fn NODE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NODE, 0)
}
fn stateModifier_all(&self) ->  Vec<Rc<StateModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stateModifier(&self, i: usize) -> Option<Rc<StateModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StateExprContextAttrs<'input> for StateExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateExpr(&mut self,)
	-> Result<Rc<StateExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_stateExpr);
        let mut _localctx: Rc<StateExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(285);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & ((1usize << (START - 80)) | (1usize << (FINAL - 80)) | (1usize << (ABSTRACT - 80)) | (1usize << (NORMAL - 80)))) != 0) {
				{
				{
				/*InvokeRule stateModifier*/
				recog.base.set_state(282);
				recog.stateModifier()?;

				}
				}
				recog.base.set_state(287);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(288);
			_la = recog.base.input.la(1);
			if { !(_la==STATE || _la==NODE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule identifier*/
			recog.base.set_state(289);
			recog.identifier()?;

			/*InvokeRule stateScope*/
			recog.base.set_state(290);
			recog.stateScope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateScope ----------------
pub type StateScopeContextAll<'input> = StateScopeContext<'input>;


pub type StateScopeContext<'input> = BaseParserRuleContext<'input,StateScopeContextExt<'input>>;

#[derive(Clone)]
pub struct StateScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StateScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StateScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stateScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StateScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stateScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateScope }
}
antlr_rust::tid!{StateScopeContextExt<'a>}

impl<'input> StateScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StateScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StateScopeContextAttrs<'input> for StateScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateScope(&mut self,)
	-> Result<Rc<StateScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_stateScope);
        let mut _localctx: Rc<StateScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(292);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(296);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & ((1usize << (LPAREN - 19)) | (1usize << (MINUS - 19)) | (1usize << (PLUS - 19)) | (1usize << (NOT - 19)) | (1usize << (BIT_NEGATION - 19)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (INITIAL - 98)) | (1usize << (FRESH - 98)) | (1usize << (ONE - 98)) | (1usize << (RETURN - 98)) | (1usize << (INTLITERAL - 98)) | (1usize << (BVLITERAL - 98)) | (1usize << (REALLITERAL - 98)) | (1usize << (CHARLITERAL - 98)) | (1usize << (STRINGLITERAL - 98)) | (1usize << (BOOLLITERAL - 98)) | (1usize << (ENUMLITERAL - 98)) | (1usize << (IDENT - 98)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(293);
				recog.statement()?;

				}
				}
				recog.base.set_state(298);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(299);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- trans ----------------
pub type TransContextAll<'input> = TransContext<'input>;


pub type TransContext<'input> = BaseParserRuleContext<'input,TransContextExt<'input>>;

#[derive(Clone)]
pub struct TransContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_trans(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_trans(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_trans(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_trans }
	//fn type_rule_index() -> usize where Self: Sized { RULE_trans }
}
antlr_rust::tid!{TransContextExt<'a>}

impl<'input> TransContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransContextExt<'input>>{

fn transScope(&self) -> Option<Rc<TransScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TRANS1
/// Returns `None` if there is no child corresponding to token TRANS1
fn TRANS1(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(TRANS1, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANS2
/// Returns `None` if there is no child corresponding to token TRANS2
fn TRANS2(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(TRANS2, 0)
}
/// Retrieves first TerminalNode corresponding to token EDGE
/// Returns `None` if there is no child corresponding to token EDGE
fn EDGE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EDGE, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TransContextAttrs<'input> for TransContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn trans(&mut self,)
	-> Result<Rc<TransContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_trans);
        let mut _localctx: Rc<TransContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(301);
			_la = recog.base.input.la(1);
			if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (TRANS1 - 65)) | (1usize << (TRANS2 - 65)) | (1usize << (EDGE - 65)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(303);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENT {
				{
				{
				/*InvokeRule identifier*/
				recog.base.set_state(302);
				recog.identifier()?;

				}
				}
			}

			/*InvokeRule transScope*/
			recog.base.set_state(305);
			recog.transScope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transScope ----------------
pub type TransScopeContextAll<'input> = TransScopeContext<'input>;


pub type TransScopeContext<'input> = BaseParserRuleContext<'input,TransScopeContextExt<'input>>;

#[derive(Clone)]
pub struct TransScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_transScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_transScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transScope }
}
antlr_rust::tid!{TransScopeContextExt<'a>}

impl<'input> TransScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn transOp(&self) -> Option<Rc<TransOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn transDef(&self) -> Option<Rc<TransDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whereExpr(&self) -> Option<Rc<WhereExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token ON
/// Returns `None` if there is no child corresponding to token ON
fn ON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ON, 0)
}
/// Retrieves first TerminalNode corresponding to token LABEL
/// Returns `None` if there is no child corresponding to token LABEL
fn LABEL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LABEL, 0)
}

}

impl<'input> TransScopeContextAttrs<'input> for TransScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transScope(&mut self,)
	-> Result<Rc<TransScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_transScope);
        let mut _localctx: Rc<TransScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(307);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(308);
			recog.identifier()?;

			{
			/*InvokeRule transOp*/
			recog.base.set_state(309);
			recog.transOp()?;

			/*InvokeRule transDef*/
			recog.base.set_state(310);
			recog.transDef()?;

			}
			recog.base.set_state(314);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ON || _la==LABEL {
				{
				recog.base.set_state(312);
				_la = recog.base.input.la(1);
				if { !(_la==ON || _la==LABEL) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule label*/
				recog.base.set_state(313);
				recog.label()?;

				}
			}

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WHERE {
				{
				/*InvokeRule whereExpr*/
				recog.base.set_state(316);
				recog.whereExpr()?;

				recog.base.set_state(317);
				recog.base.match_token(SEMI,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(321);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transOp ----------------
pub type TransOpContextAll<'input> = TransOpContext<'input>;


pub type TransOpContext<'input> = BaseParserRuleContext<'input,TransOpContextExt<'input>>;

#[derive(Clone)]
pub struct TransOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransOpContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransOpContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transOp(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_transOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_transOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transOp }
}
antlr_rust::tid!{TransOpContextExt<'a>}

impl<'input> TransOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransOpContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
/// Retrieves first TerminalNode corresponding to token BI_ARROW
/// Returns `None` if there is no child corresponding to token BI_ARROW
fn BI_ARROW(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BI_ARROW, 0)
}

}

impl<'input> TransOpContextAttrs<'input> for TransOpContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transOp(&mut self,)
	-> Result<Rc<TransOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_transOp);
        let mut _localctx: Rc<TransOpContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(323);
			_la = recog.base.input.la(1);
			if { !(_la==ARROW || _la==BI_ARROW) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transDef ----------------
pub type TransDefContextAll<'input> = TransDefContext<'input>;


pub type TransDefContext<'input> = BaseParserRuleContext<'input,TransDefContextExt<'input>>;

#[derive(Clone)]
pub struct TransDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransDefContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransDefContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transDef(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_transDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_transDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transDef }
}
antlr_rust::tid!{TransDefContextExt<'a>}

impl<'input> TransDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransDefContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransDefContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
/// Retrieves first TerminalNode corresponding to token STAR
/// Returns `None` if there is no child corresponding to token STAR
fn STAR(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STAR, 0)
}
fn transExclExpr(&self) -> Option<Rc<TransExclExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}

}

impl<'input> TransDefContextAttrs<'input> for TransDefContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transDef(&mut self,)
	-> Result<Rc<TransDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_transDef);
        let mut _localctx: Rc<TransDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(341);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(325);
					recog.identifier()?;

					recog.base.set_state(330);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(326);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule identifier*/
						recog.base.set_state(327);
						recog.identifier()?;

						}
						}
						recog.base.set_state(332);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 STAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(333);
					recog.base.match_token(STAR,&mut recog.err_handler)?;

					recog.base.set_state(335);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACK {
						{
						/*InvokeRule transExclExpr*/
						recog.base.set_state(334);
						recog.transExclExpr()?;

						}
					}

					}
				}

			 PLUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(337);
					recog.base.match_token(PLUS,&mut recog.err_handler)?;

					recog.base.set_state(339);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LBRACK {
						{
						/*InvokeRule transExclExpr*/
						recog.base.set_state(338);
						recog.transExclExpr()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- transExclExpr ----------------
pub type TransExclExprContextAll<'input> = TransExclExprContext<'input>;


pub type TransExclExprContext<'input> = BaseParserRuleContext<'input,TransExclExprContextExt<'input>>;

#[derive(Clone)]
pub struct TransExclExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for TransExclExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for TransExclExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_transExclExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_transExclExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for TransExclExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_transExclExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransExclExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_transExclExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_transExclExpr }
}
antlr_rust::tid!{TransExclExprContextExt<'a>}

impl<'input> TransExclExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TransExclExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TransExclExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TransExclExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<TransExclExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TransExclExprContextAttrs<'input> for TransExclExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn transExclExpr(&mut self,)
	-> Result<Rc<TransExclExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TransExclExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_transExclExpr);
        let mut _localctx: Rc<TransExclExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(343);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(344);
			recog.identifier()?;

			recog.base.set_state(349);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(345);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(346);
				recog.identifier()?;

				}
				}
				recog.base.set_state(351);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(352);
			recog.base.match_token(RBRACK,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- invariantExpression ----------------
pub type InvariantExpressionContextAll<'input> = InvariantExpressionContext<'input>;


pub type InvariantExpressionContext<'input> = BaseParserRuleContext<'input,InvariantExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct InvariantExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for InvariantExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for InvariantExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_invariantExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_invariantExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for InvariantExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_invariantExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for InvariantExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_invariantExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_invariantExpression }
}
antlr_rust::tid!{InvariantExpressionContextExt<'a>}

impl<'input> InvariantExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InvariantExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InvariantExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InvariantExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<InvariantExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INVARIANT
/// Returns `None` if there is no child corresponding to token INVARIANT
fn INVARIANT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(INVARIANT, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn invariantScope(&self) -> Option<Rc<InvariantScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inExpr(&self) -> Option<Rc<InExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InvariantExpressionContextAttrs<'input> for InvariantExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn invariantExpression(&mut self,)
	-> Result<Rc<InvariantExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InvariantExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_invariantExpression);
        let mut _localctx: Rc<InvariantExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(354);
			recog.base.match_token(INVARIANT,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(355);
			recog.identifier()?;

			/*InvokeRule invariantScope*/
			recog.base.set_state(356);
			recog.invariantScope()?;

			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IN {
				{
				/*InvokeRule inExpr*/
				recog.base.set_state(357);
				recog.inExpr()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inExpr ----------------
pub type InExprContextAll<'input> = InExprContext<'input>;


pub type InExprContext<'input> = BaseParserRuleContext<'input,InExprContextExt<'input>>;

#[derive(Clone)]
pub struct InExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for InExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for InExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_inExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for InExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_inExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for InExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inExpr }
}
antlr_rust::tid!{InExprContextExt<'a>}

impl<'input> InExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<InExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> InExprContextAttrs<'input> for InExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inExpr(&mut self,)
	-> Result<Rc<InExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_inExpr);
        let mut _localctx: Rc<InExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(360);
			recog.base.match_token(IN,&mut recog.err_handler)?;

			recog.base.set_state(361);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(362);
			recog.identifier()?;

			recog.base.set_state(367);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(363);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(364);
				recog.identifier()?;

				}
				}
				recog.base.set_state(369);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(370);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- invariantScope ----------------
pub type InvariantScopeContextAll<'input> = InvariantScopeContext<'input>;


pub type InvariantScopeContext<'input> = BaseParserRuleContext<'input,InvariantScopeContextExt<'input>>;

#[derive(Clone)]
pub struct InvariantScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for InvariantScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for InvariantScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_invariantScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_invariantScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for InvariantScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_invariantScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for InvariantScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_invariantScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_invariantScope }
}
antlr_rust::tid!{InvariantScopeContextExt<'a>}

impl<'input> InvariantScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InvariantScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InvariantScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InvariantScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<InvariantScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InvariantScopeContextAttrs<'input> for InvariantScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn invariantScope(&mut self,)
	-> Result<Rc<InvariantScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InvariantScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_invariantScope);
        let mut _localctx: Rc<InvariantScopeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(372);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			{
			/*InvokeRule statement*/
			recog.base.set_state(373);
			recog.statement()?;

			}
			recog.base.set_state(374);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- goal ----------------
pub type GoalContextAll<'input> = GoalContext<'input>;


pub type GoalContext<'input> = BaseParserRuleContext<'input,GoalContextExt<'input>>;

#[derive(Clone)]
pub struct GoalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for GoalContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for GoalContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_goal(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_goal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for GoalContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_goal(self);
	}
}

impl<'input> CustomRuleContext<'input> for GoalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_goal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_goal }
}
antlr_rust::tid!{GoalContextExt<'a>}

impl<'input> GoalContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GoalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GoalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GoalContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<GoalContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GOAL
/// Returns `None` if there is no child corresponding to token GOAL
fn GOAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(GOAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn checkExpr(&self) -> Option<Rc<CheckExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn letExpr_all(&self) ->  Vec<Rc<LetExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn letExpr(&self, i: usize) -> Option<Rc<LetExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn pathAssignStatement_all(&self) ->  Vec<Rc<PathAssignStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pathAssignStatement(&self, i: usize) -> Option<Rc<PathAssignStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn assertExpr_all(&self) ->  Vec<Rc<AssertExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assertExpr(&self, i: usize) -> Option<Rc<AssertExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> GoalContextAttrs<'input> for GoalContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn goal(&mut self,)
	-> Result<Rc<GoalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GoalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_goal);
        let mut _localctx: Rc<GoalContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(376);
			recog.base.match_token(GOAL,&mut recog.err_handler)?;

			recog.base.set_state(377);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(383);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AT_SIGN || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (LET - 95)) | (1usize << (ASSERT - 95)) | (1usize << (IDENT - 95)))) != 0) {
				{
				recog.base.set_state(381);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 LET 
					=> {
						{
						{
						/*InvokeRule letExpr*/
						recog.base.set_state(378);
						recog.letExpr()?;

						}
						}
					}

				 IDENT 
					=> {
						{
						{
						/*InvokeRule pathAssignStatement*/
						recog.base.set_state(379);
						recog.pathAssignStatement()?;

						}
						}
					}

				 AT_SIGN | ASSERT 
					=> {
						{
						{
						/*InvokeRule assertExpr*/
						recog.base.set_state(380);
						recog.assertExpr()?;

						}
						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(385);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule checkExpr*/
			recog.base.set_state(386);
			recog.checkExpr()?;

			recog.base.set_state(387);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- checkExpr ----------------
pub type CheckExprContextAll<'input> = CheckExprContext<'input>;


pub type CheckExprContext<'input> = BaseParserRuleContext<'input,CheckExprContextExt<'input>>;

#[derive(Clone)]
pub struct CheckExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for CheckExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for CheckExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_checkExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_checkExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for CheckExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_checkExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CheckExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_checkExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_checkExpr }
}
antlr_rust::tid!{CheckExprContextExt<'a>}

impl<'input> CheckExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CheckExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CheckExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CheckExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<CheckExprContextExt<'input>>{

fn forExpr(&self) -> Option<Rc<ForExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CHECK
/// Returns `None` if there is no child corresponding to token CHECK
fn CHECK(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(CHECK, 0)
}
/// Retrieves first TerminalNode corresponding to token ENUMERATE
/// Returns `None` if there is no child corresponding to token ENUMERATE
fn ENUMERATE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ENUMERATE, 0)
}
fn viaExpr(&self) -> Option<Rc<ViaExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn withExpr(&self) -> Option<Rc<WithExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stopExpr(&self) -> Option<Rc<StopExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CheckExprContextAttrs<'input> for CheckExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn checkExpr(&mut self,)
	-> Result<Rc<CheckExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CheckExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_checkExpr);
        let mut _localctx: Rc<CheckExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(389);
			_la = recog.base.input.la(1);
			if { !(_la==CHECK || _la==ENUMERATE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule forExpr*/
			recog.base.set_state(390);
			recog.forExpr()?;

			recog.base.set_state(392);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==VIA || _la==CONDITION {
				{
				/*InvokeRule viaExpr*/
				recog.base.set_state(391);
				recog.viaExpr()?;

				}
			}

			recog.base.set_state(395);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WITH {
				{
				/*InvokeRule withExpr*/
				recog.base.set_state(394);
				recog.withExpr()?;

				}
			}

			recog.base.set_state(398);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STOP || _la==REACH {
				{
				/*InvokeRule stopExpr*/
				recog.base.set_state(397);
				recog.stopExpr()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forExpr ----------------
pub type ForExprContextAll<'input> = ForExprContext<'input>;


pub type ForExprContext<'input> = BaseParserRuleContext<'input,ForExprContextExt<'input>>;

#[derive(Clone)]
pub struct ForExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ForExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ForExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_forExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ForExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_forExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forExpr }
}
antlr_rust::tid!{ForExprContextExt<'a>}

impl<'input> ForExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ForExprContextExt<'input>>{

fn intLiteral_all(&self) ->  Vec<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn intLiteral(&self, i: usize) -> Option<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token EACH
/// Returns `None` if there is no child corresponding to token EACH
fn EACH(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EACH, 0)
}
/// Retrieves first TerminalNode corresponding to token UPTO
/// Returns `None` if there is no child corresponding to token UPTO
fn UPTO(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(UPTO, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ForExprContextAttrs<'input> for ForExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forExpr(&mut self,)
	-> Result<Rc<ForExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_forExpr);
        let mut _localctx: Rc<ForExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(400);
			_la = recog.base.input.la(1);
			if { !(((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (FOR - 87)) | (1usize << (EACH - 87)) | (1usize << (UPTO - 87)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule intLiteral*/
			recog.base.set_state(401);
			recog.intLiteral()?;

			recog.base.set_state(406);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(402);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule intLiteral*/
				recog.base.set_state(403);
				recog.intLiteral()?;

				}
				}
				recog.base.set_state(408);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stopExpr ----------------
pub type StopExprContextAll<'input> = StopExprContext<'input>;


pub type StopExprContext<'input> = BaseParserRuleContext<'input,StopExprContextExt<'input>>;

#[derive(Clone)]
pub struct StopExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StopExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StopExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stopExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stopExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StopExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stopExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for StopExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stopExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stopExpr }
}
antlr_rust::tid!{StopExprContextExt<'a>}

impl<'input> StopExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StopExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StopExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StopExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StopExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token REACH
/// Returns `None` if there is no child corresponding to token REACH
fn REACH(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(REACH, 0)
}
/// Retrieves first TerminalNode corresponding to token STOP
/// Returns `None` if there is no child corresponding to token STOP
fn STOP(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STOP, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> StopExprContextAttrs<'input> for StopExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stopExpr(&mut self,)
	-> Result<Rc<StopExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StopExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_stopExpr);
        let mut _localctx: Rc<StopExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(409);
			_la = recog.base.input.la(1);
			if { !(_la==STOP || _la==REACH) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(410);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(411);
			recog.identifier()?;

			recog.base.set_state(416);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(412);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(413);
				recog.identifier()?;

				}
				}
				recog.base.set_state(418);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(419);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- viaExpr ----------------
pub type ViaExprContextAll<'input> = ViaExprContext<'input>;


pub type ViaExprContext<'input> = BaseParserRuleContext<'input,ViaExprContextExt<'input>>;

#[derive(Clone)]
pub struct ViaExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ViaExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ViaExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_viaExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_viaExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ViaExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_viaExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ViaExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_viaExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_viaExpr }
}
antlr_rust::tid!{ViaExprContextExt<'a>}

impl<'input> ViaExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ViaExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ViaExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ViaExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ViaExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn pathExprList(&self) -> Option<Rc<PathExprListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token VIA
/// Returns `None` if there is no child corresponding to token VIA
fn VIA(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(VIA, 0)
}
/// Retrieves first TerminalNode corresponding to token CONDITION
/// Returns `None` if there is no child corresponding to token CONDITION
fn CONDITION(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(CONDITION, 0)
}

}

impl<'input> ViaExprContextAttrs<'input> for ViaExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn viaExpr(&mut self,)
	-> Result<Rc<ViaExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ViaExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_viaExpr);
        let mut _localctx: Rc<ViaExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(421);
			_la = recog.base.input.la(1);
			if { !(_la==VIA || _la==CONDITION) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(422);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule pathExprList*/
			recog.base.set_state(423);
			recog.pathExprList()?;

			recog.base.set_state(424);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathExprList ----------------
pub type PathExprListContextAll<'input> = PathExprListContext<'input>;


pub type PathExprListContext<'input> = BaseParserRuleContext<'input,PathExprListContextExt<'input>>;

#[derive(Clone)]
pub struct PathExprListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathExprListContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathExprListContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathExprList(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathExprList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathExprListContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathExprList(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathExprListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathExprList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathExprList }
}
antlr_rust::tid!{PathExprListContextExt<'a>}

impl<'input> PathExprListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathExprListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathExprListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathExprListContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathExprListContextExt<'input>>{

fn pathExpr_all(&self) ->  Vec<Rc<PathExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pathExpr(&self, i: usize) -> Option<Rc<PathExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> PathExprListContextAttrs<'input> for PathExprListContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathExprList(&mut self,)
	-> Result<Rc<PathExprListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathExprListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_pathExprList);
        let mut _localctx: Rc<PathExprListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pathExpr*/
			recog.base.set_state(426);
			recog.pathExpr()?;

			recog.base.set_state(431);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(427);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule pathExpr*/
				recog.base.set_state(428);
				recog.pathExpr()?;

				}
				}
				recog.base.set_state(433);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- withExpr ----------------
pub type WithExprContextAll<'input> = WithExprContext<'input>;


pub type WithExprContext<'input> = BaseParserRuleContext<'input,WithExprContextExt<'input>>;

#[derive(Clone)]
pub struct WithExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for WithExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for WithExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_withExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_withExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for WithExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_withExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for WithExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_withExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_withExpr }
}
antlr_rust::tid!{WithExprContextExt<'a>}

impl<'input> WithExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WithExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WithExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WithExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<WithExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> WithExprContextAttrs<'input> for WithExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn withExpr(&mut self,)
	-> Result<Rc<WithExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WithExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_withExpr);
        let mut _localctx: Rc<WithExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(434);
			recog.base.match_token(WITH,&mut recog.err_handler)?;

			recog.base.set_state(435);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			{
			/*InvokeRule identifier*/
			recog.base.set_state(436);
			recog.identifier()?;

			recog.base.set_state(441);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(437);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(438);
				recog.identifier()?;

				}
				}
				recog.base.set_state(443);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			recog.base.set_state(444);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- letExpr ----------------
pub type LetExprContextAll<'input> = LetExprContext<'input>;


pub type LetExprContext<'input> = BaseParserRuleContext<'input,LetExprContextExt<'input>>;

#[derive(Clone)]
pub struct LetExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for LetExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for LetExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_letExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_letExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for LetExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_letExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for LetExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_letExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_letExpr }
}
antlr_rust::tid!{LetExprContextExt<'a>}

impl<'input> LetExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LetExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LetExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LetExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<LetExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LET
/// Returns `None` if there is no child corresponding to token LET
fn LET(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LET, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn pathCondAssignExpr(&self) -> Option<Rc<PathCondAssignExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LetExprContextAttrs<'input> for LetExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn letExpr(&mut self,)
	-> Result<Rc<LetExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LetExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_letExpr);
        let mut _localctx: Rc<LetExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(446);
			recog.base.match_token(LET,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(447);
			recog.identifier()?;

			recog.base.set_state(449);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EQUAL {
				{
				/*InvokeRule pathCondAssignExpr*/
				recog.base.set_state(448);
				recog.pathCondAssignExpr()?;

				}
			}

			recog.base.set_state(451);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathAssignStatement ----------------
pub type PathAssignStatementContextAll<'input> = PathAssignStatementContext<'input>;


pub type PathAssignStatementContext<'input> = BaseParserRuleContext<'input,PathAssignStatementContextExt<'input>>;

#[derive(Clone)]
pub struct PathAssignStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathAssignStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathAssignStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathAssignStatement(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathAssignStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathAssignStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathAssignStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathAssignStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathAssignStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathAssignStatement }
}
antlr_rust::tid!{PathAssignStatementContextExt<'a>}

impl<'input> PathAssignStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathAssignStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathAssignStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathAssignStatementContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathAssignStatementContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pathCondAssignExpr(&self) -> Option<Rc<PathCondAssignExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> PathAssignStatementContextAttrs<'input> for PathAssignStatementContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathAssignStatement(&mut self,)
	-> Result<Rc<PathAssignStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathAssignStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_pathAssignStatement);
        let mut _localctx: Rc<PathAssignStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(453);
			recog.identifier()?;

			/*InvokeRule pathCondAssignExpr*/
			recog.base.set_state(454);
			recog.pathCondAssignExpr()?;

			recog.base.set_state(455);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathCondAssignExpr ----------------
pub type PathCondAssignExprContextAll<'input> = PathCondAssignExprContext<'input>;


pub type PathCondAssignExprContext<'input> = BaseParserRuleContext<'input,PathCondAssignExprContextExt<'input>>;

#[derive(Clone)]
pub struct PathCondAssignExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathCondAssignExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathCondAssignExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathCondAssignExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathCondAssignExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathCondAssignExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathCondAssignExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathCondAssignExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathCondAssignExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathCondAssignExpr }
}
antlr_rust::tid!{PathCondAssignExprContextExt<'a>}

impl<'input> PathCondAssignExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathCondAssignExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathCondAssignExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathCondAssignExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathCondAssignExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
fn pathCondition(&self) -> Option<Rc<PathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PathCondAssignExprContextAttrs<'input> for PathCondAssignExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathCondAssignExpr(&mut self,)
	-> Result<Rc<PathCondAssignExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathCondAssignExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_pathCondAssignExpr);
        let mut _localctx: Rc<PathCondAssignExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(457);
			recog.base.match_token(EQUAL,&mut recog.err_handler)?;

			/*InvokeRule pathCondition*/
			recog.base.set_state(458);
			recog.pathCondition()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathExpr ----------------
pub type PathExprContextAll<'input> = PathExprContext<'input>;


pub type PathExprContext<'input> = BaseParserRuleContext<'input,PathExprContextExt<'input>>;

#[derive(Clone)]
pub struct PathExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathExpr }
}
antlr_rust::tid!{PathExprContextExt<'a>}

impl<'input> PathExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathExprContextExt<'input>>{

fn pathCondition(&self) -> Option<Rc<PathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PathExprContextAttrs<'input> for PathExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathExpr(&mut self,)
	-> Result<Rc<PathExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_pathExpr);
        let mut _localctx: Rc<PathExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pathCondition*/
			recog.base.set_state(460);
			recog.pathCondition()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathCondition ----------------
pub type PathConditionContextAll<'input> = PathConditionContext<'input>;


pub type PathConditionContext<'input> = BaseParserRuleContext<'input,PathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct PathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathCondition }
}
antlr_rust::tid!{PathConditionContextExt<'a>}

impl<'input> PathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathConditionContextExt<'input>>{

fn orPathCondition(&self) -> Option<Rc<OrPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PathConditionContextAttrs<'input> for PathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathCondition(&mut self,)
	-> Result<Rc<PathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_pathCondition);
        let mut _localctx: Rc<PathConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule orPathCondition*/
			recog.base.set_state(462);
			recog.orPathCondition()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- orPathCondition ----------------
pub type OrPathConditionContextAll<'input> = OrPathConditionContext<'input>;


pub type OrPathConditionContext<'input> = BaseParserRuleContext<'input,OrPathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct OrPathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for OrPathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for OrPathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_orPathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_orPathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for OrPathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_orPathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrPathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_orPathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_orPathCondition }
}
antlr_rust::tid!{OrPathConditionContextExt<'a>}

impl<'input> OrPathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OrPathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OrPathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OrPathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<OrPathConditionContextExt<'input>>{

fn andPathCondition_all(&self) ->  Vec<Rc<AndPathConditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn andPathCondition(&self, i: usize) -> Option<Rc<AndPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> OrPathConditionContextAttrs<'input> for OrPathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn orPathCondition(&mut self,)
	-> Result<Rc<OrPathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OrPathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_orPathCondition);
        let mut _localctx: Rc<OrPathConditionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule andPathCondition*/
			recog.base.set_state(464);
			recog.andPathCondition()?;

			recog.base.set_state(469);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OR {
				{
				{
				recog.base.set_state(465);
				recog.base.match_token(OR,&mut recog.err_handler)?;

				/*InvokeRule andPathCondition*/
				recog.base.set_state(466);
				recog.andPathCondition()?;

				}
				}
				recog.base.set_state(471);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- andPathCondition ----------------
pub type AndPathConditionContextAll<'input> = AndPathConditionContext<'input>;


pub type AndPathConditionContext<'input> = BaseParserRuleContext<'input,AndPathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct AndPathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for AndPathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for AndPathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_andPathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_andPathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for AndPathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_andPathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndPathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_andPathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_andPathCondition }
}
antlr_rust::tid!{AndPathConditionContextExt<'a>}

impl<'input> AndPathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AndPathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AndPathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AndPathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<AndPathConditionContextExt<'input>>{

fn xorPathCondition_all(&self) ->  Vec<Rc<XorPathConditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn xorPathCondition(&self, i: usize) -> Option<Rc<XorPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}

}

impl<'input> AndPathConditionContextAttrs<'input> for AndPathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn andPathCondition(&mut self,)
	-> Result<Rc<AndPathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AndPathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_andPathCondition);
        let mut _localctx: Rc<AndPathConditionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule xorPathCondition*/
			recog.base.set_state(472);
			recog.xorPathCondition()?;

			recog.base.set_state(477);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AND {
				{
				{
				recog.base.set_state(473);
				recog.base.match_token(AND,&mut recog.err_handler)?;

				/*InvokeRule xorPathCondition*/
				recog.base.set_state(474);
				recog.xorPathCondition()?;

				}
				}
				recog.base.set_state(479);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- xorPathCondition ----------------
pub type XorPathConditionContextAll<'input> = XorPathConditionContext<'input>;


pub type XorPathConditionContext<'input> = BaseParserRuleContext<'input,XorPathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct XorPathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for XorPathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for XorPathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_xorPathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_xorPathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for XorPathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_xorPathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for XorPathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_xorPathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_xorPathCondition }
}
antlr_rust::tid!{XorPathConditionContextExt<'a>}

impl<'input> XorPathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<XorPathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,XorPathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait XorPathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<XorPathConditionContextExt<'input>>{

fn unaryPathCondition_all(&self) ->  Vec<Rc<UnaryPathConditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unaryPathCondition(&self, i: usize) -> Option<Rc<UnaryPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token HAT in current rule
fn HAT_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token HAT, starting from 0.
/// Returns `None` if number of children corresponding to token HAT is less or equal than `i`.
fn HAT(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(HAT, i)
}

}

impl<'input> XorPathConditionContextAttrs<'input> for XorPathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn xorPathCondition(&mut self,)
	-> Result<Rc<XorPathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = XorPathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_xorPathCondition);
        let mut _localctx: Rc<XorPathConditionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryPathCondition*/
			recog.base.set_state(480);
			recog.unaryPathCondition()?;

			recog.base.set_state(485);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==HAT {
				{
				{
				recog.base.set_state(481);
				recog.base.match_token(HAT,&mut recog.err_handler)?;

				/*InvokeRule unaryPathCondition*/
				recog.base.set_state(482);
				recog.unaryPathCondition()?;

				}
				}
				recog.base.set_state(487);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryPathCondition ----------------
pub type UnaryPathConditionContextAll<'input> = UnaryPathConditionContext<'input>;


pub type UnaryPathConditionContext<'input> = BaseParserRuleContext<'input,UnaryPathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryPathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for UnaryPathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for UnaryPathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryPathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_unaryPathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for UnaryPathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_unaryPathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryPathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryPathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryPathCondition }
}
antlr_rust::tid!{UnaryPathConditionContextExt<'a>}

impl<'input> UnaryPathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryPathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryPathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryPathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<UnaryPathConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn unaryPathCondition(&self) -> Option<Rc<UnaryPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn primaryCondition(&self) -> Option<Rc<PrimaryConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parPathCondition(&self) -> Option<Rc<ParPathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnaryPathConditionContextAttrs<'input> for UnaryPathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryPathCondition(&mut self,)
	-> Result<Rc<UnaryPathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryPathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_unaryPathCondition);
        let mut _localctx: Rc<UnaryPathConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(492);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(488);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule unaryPathCondition*/
					recog.base.set_state(489);
					recog.unaryPathCondition()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule primaryCondition*/
					recog.base.set_state(490);
					recog.primaryCondition()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule parPathCondition*/
					recog.base.set_state(491);
					recog.parPathCondition()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primaryCondition ----------------
pub type PrimaryConditionContextAll<'input> = PrimaryConditionContext<'input>;


pub type PrimaryConditionContext<'input> = BaseParserRuleContext<'input,PrimaryConditionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PrimaryConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PrimaryConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primaryCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_primaryCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PrimaryConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_primaryCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryCondition }
}
antlr_rust::tid!{PrimaryConditionContextExt<'a>}

impl<'input> PrimaryConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PrimaryConditionContextExt<'input>>{

fn stateIncExpr(&self) -> Option<Rc<StateIncExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pathPrimaryExpr(&self) -> Option<Rc<PathPrimaryExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn boolLiteral(&self) -> Option<Rc<BoolLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryConditionContextAttrs<'input> for PrimaryConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primaryCondition(&mut self,)
	-> Result<Rc<PrimaryConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_primaryCondition);
        let mut _localctx: Rc<PrimaryConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(497);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stateIncExpr*/
					recog.base.set_state(494);
					recog.stateIncExpr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule pathPrimaryExpr*/
					recog.base.set_state(495);
					recog.pathPrimaryExpr()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule boolLiteral*/
					recog.base.set_state(496);
					recog.boolLiteral()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parPathCondition ----------------
pub type ParPathConditionContextAll<'input> = ParPathConditionContext<'input>;


pub type ParPathConditionContext<'input> = BaseParserRuleContext<'input,ParPathConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ParPathConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ParPathConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ParPathConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parPathCondition(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_parPathCondition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ParPathConditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_parPathCondition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParPathConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parPathCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parPathCondition }
}
antlr_rust::tid!{ParPathConditionContextExt<'a>}

impl<'input> ParPathConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParPathConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParPathConditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParPathConditionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ParPathConditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn pathCondition(&self) -> Option<Rc<PathConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ParPathConditionContextAttrs<'input> for ParPathConditionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parPathCondition(&mut self,)
	-> Result<Rc<ParPathConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParPathConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_parPathCondition);
        let mut _localctx: Rc<ParPathConditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(499);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule pathCondition*/
			recog.base.set_state(500);
			recog.pathCondition()?;

			recog.base.set_state(501);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateIncExpr ----------------
pub type StateIncExprContextAll<'input> = StateIncExprContext<'input>;


pub type StateIncExprContext<'input> = BaseParserRuleContext<'input,StateIncExprContextExt<'input>>;

#[derive(Clone)]
pub struct StateIncExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StateIncExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StateIncExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateIncExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stateIncExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StateIncExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stateIncExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateIncExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateIncExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateIncExpr }
}
antlr_rust::tid!{StateIncExprContextExt<'a>}

impl<'input> StateIncExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateIncExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateIncExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateIncExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StateIncExprContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SHIFT_LEFT
/// Returns `None` if there is no child corresponding to token SHIFT_LEFT
fn SHIFT_LEFT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_LEFT, 0)
}
/// Retrieves first TerminalNode corresponding to token SHIFT_RIGHT
/// Returns `None` if there is no child corresponding to token SHIFT_RIGHT
fn SHIFT_RIGHT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_RIGHT, 0)
}
/// Retrieves first TerminalNode corresponding to token HAT
/// Returns `None` if there is no child corresponding to token HAT
fn HAT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(HAT, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn intLiteral_all(&self) ->  Vec<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn intLiteral(&self, i: usize) -> Option<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> StateIncExprContextAttrs<'input> for StateIncExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateIncExpr(&mut self,)
	-> Result<Rc<StateIncExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateIncExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_stateIncExpr);
        let mut _localctx: Rc<StateIncExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(550);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(511);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 SHIFT_LEFT 
						=> {
					    	{
					    	recog.base.set_state(503);
					    	recog.base.match_token(SHIFT_LEFT,&mut recog.err_handler)?;

					    	recog.base.set_state(505);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(504);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 SHIFT_RIGHT 
						=> {
					    	{
					    	recog.base.set_state(507);
					    	recog.base.match_token(SHIFT_RIGHT,&mut recog.err_handler)?;

					    	recog.base.set_state(509);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(508);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 IDENT 
						=> {
					    }

						_ => {}
					}
					/*InvokeRule identifier*/
					recog.base.set_state(513);
					recog.identifier()?;

					recog.base.set_state(523);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(514);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							recog.base.set_state(515);
							recog.base.match_token(LBRACE,&mut recog.err_handler)?;

							/*InvokeRule intLiteral*/
							recog.base.set_state(516);
							recog.intLiteral()?;

							recog.base.set_state(519);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==COLON {
								{
								recog.base.set_state(517);
								recog.base.match_token(COLON,&mut recog.err_handler)?;

								/*InvokeRule intLiteral*/
								recog.base.set_state(518);
								recog.intLiteral()?;

								}
							}

							recog.base.set_state(521);
							recog.base.match_token(RBRACE,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					{
					recog.base.set_state(533);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 SHIFT_LEFT 
						=> {
					    	{
					    	recog.base.set_state(525);
					    	recog.base.match_token(SHIFT_LEFT,&mut recog.err_handler)?;

					    	recog.base.set_state(527);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(526);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 SHIFT_RIGHT 
						=> {
					    	{
					    	recog.base.set_state(529);
					    	recog.base.match_token(SHIFT_RIGHT,&mut recog.err_handler)?;

					    	recog.base.set_state(531);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(530);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 LPAREN 
						=> {
					    }

						_ => {}
					}
					recog.base.set_state(535);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule identifier*/
					recog.base.set_state(536);
					recog.identifier()?;

					recog.base.set_state(546);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==HAT {
						{
						recog.base.set_state(537);
						recog.base.match_token(HAT,&mut recog.err_handler)?;

						recog.base.set_state(538);
						recog.base.match_token(LBRACE,&mut recog.err_handler)?;

						/*InvokeRule intLiteral*/
						recog.base.set_state(539);
						recog.intLiteral()?;

						recog.base.set_state(542);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==COLON {
							{
							recog.base.set_state(540);
							recog.base.match_token(COLON,&mut recog.err_handler)?;

							/*InvokeRule intLiteral*/
							recog.base.set_state(541);
							recog.intLiteral()?;

							}
						}

						recog.base.set_state(544);
						recog.base.match_token(RBRACE,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(548);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathPrimaryExpr ----------------
pub type PathPrimaryExprContextAll<'input> = PathPrimaryExprContext<'input>;


pub type PathPrimaryExprContext<'input> = BaseParserRuleContext<'input,PathPrimaryExprContextExt<'input>>;

#[derive(Clone)]
pub struct PathPrimaryExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathPrimaryExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathPrimaryExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathPrimaryExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathPrimaryExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathPrimaryExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathPrimaryExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathPrimaryExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathPrimaryExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathPrimaryExpr }
}
antlr_rust::tid!{PathPrimaryExprContextExt<'a>}

impl<'input> PathPrimaryExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathPrimaryExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathPrimaryExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathPrimaryExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathPrimaryExprContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn pathOp_all(&self) ->  Vec<Rc<PathOpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pathOp(&self, i: usize) -> Option<Rc<PathOpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token ARROW in current rule
fn ARROW_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ARROW, starting from 0.
/// Returns `None` if number of children corresponding to token ARROW is less or equal than `i`.
fn ARROW(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ARROW, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token SHIFT_LEFT
/// Returns `None` if there is no child corresponding to token SHIFT_LEFT
fn SHIFT_LEFT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_LEFT, 0)
}
/// Retrieves first TerminalNode corresponding to token SHIFT_RIGHT
/// Returns `None` if there is no child corresponding to token SHIFT_RIGHT
fn SHIFT_RIGHT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_RIGHT, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token HAT
/// Returns `None` if there is no child corresponding to token HAT
fn HAT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(HAT, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn intLiteral_all(&self) ->  Vec<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn intLiteral(&self, i: usize) -> Option<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> PathPrimaryExprContextAttrs<'input> for PathPrimaryExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathPrimaryExpr(&mut self,)
	-> Result<Rc<PathPrimaryExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathPrimaryExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_pathPrimaryExpr);
        let mut _localctx: Rc<PathPrimaryExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(654);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 P_OP_ONE | IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					recog.base.set_state(567);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 IDENT 
						=> {
							{
							/*InvokeRule identifier*/
							recog.base.set_state(552);
							recog.identifier()?;

							}
						}

					 P_OP_ONE 
						=> {
							{
							/*InvokeRule pathOp*/
							recog.base.set_state(553);
							recog.pathOp()?;

							recog.base.set_state(565);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==LBRACK {
								{
								recog.base.set_state(554);
								recog.base.match_token(LBRACK,&mut recog.err_handler)?;

								/*InvokeRule identifier*/
								recog.base.set_state(555);
								recog.identifier()?;

								recog.base.set_state(560);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==COMMA {
									{
									{
									recog.base.set_state(556);
									recog.base.match_token(COMMA,&mut recog.err_handler)?;

									/*InvokeRule identifier*/
									recog.base.set_state(557);
									recog.identifier()?;

									}
									}
									recog.base.set_state(562);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								recog.base.set_state(563);
								recog.base.match_token(RBRACK,&mut recog.err_handler)?;

								}
							}

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(587); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(569);
						recog.base.match_token(ARROW,&mut recog.err_handler)?;

						recog.base.set_state(585);
						recog.err_handler.sync(&mut recog.base)?;
						match recog.base.input.la(1) {
						 IDENT 
							=> {
								{
								/*InvokeRule identifier*/
								recog.base.set_state(570);
								recog.identifier()?;

								}
							}

						 P_OP_ONE 
							=> {
								{
								/*InvokeRule pathOp*/
								recog.base.set_state(571);
								recog.pathOp()?;

								recog.base.set_state(583);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if _la==LBRACK {
									{
									recog.base.set_state(572);
									recog.base.match_token(LBRACK,&mut recog.err_handler)?;

									/*InvokeRule identifier*/
									recog.base.set_state(573);
									recog.identifier()?;

									recog.base.set_state(578);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
									while _la==COMMA {
										{
										{
										recog.base.set_state(574);
										recog.base.match_token(COMMA,&mut recog.err_handler)?;

										/*InvokeRule identifier*/
										recog.base.set_state(575);
										recog.identifier()?;

										}
										}
										recog.base.set_state(580);
										recog.err_handler.sync(&mut recog.base)?;
										_la = recog.base.input.la(1);
									}
									recog.base.set_state(581);
									recog.base.match_token(RBRACK,&mut recog.err_handler)?;

									}
								}

								}
							}

							_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						}
						}
						recog.base.set_state(589); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==ARROW) {break}
					}
					}
					}
				}

			 LPAREN | SHIFT_LEFT | SHIFT_RIGHT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					{
					recog.base.set_state(599);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 SHIFT_LEFT 
						=> {
					    	{
					    	recog.base.set_state(591);
					    	recog.base.match_token(SHIFT_LEFT,&mut recog.err_handler)?;

					    	recog.base.set_state(593);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(592);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 SHIFT_RIGHT 
						=> {
					    	{
					    	recog.base.set_state(595);
					    	recog.base.match_token(SHIFT_RIGHT,&mut recog.err_handler)?;

					    	recog.base.set_state(597);
					    	recog.err_handler.sync(&mut recog.base)?;
					    	_la = recog.base.input.la(1);
					    	if _la==INTLITERAL {
					    		{
					    		/*InvokeRule intLiteral*/
					    		recog.base.set_state(596);
					    		recog.intLiteral()?;

					    		}
					    	}

					    	}
					    }

					 LPAREN 
						=> {
					    }

						_ => {}
					}
					recog.base.set_state(601);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(617);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 IDENT 
						=> {
							{
							/*InvokeRule identifier*/
							recog.base.set_state(602);
							recog.identifier()?;

							}
						}

					 P_OP_ONE 
						=> {
							{
							/*InvokeRule pathOp*/
							recog.base.set_state(603);
							recog.pathOp()?;

							recog.base.set_state(615);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==LBRACK {
								{
								recog.base.set_state(604);
								recog.base.match_token(LBRACK,&mut recog.err_handler)?;

								/*InvokeRule identifier*/
								recog.base.set_state(605);
								recog.identifier()?;

								recog.base.set_state(610);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==COMMA {
									{
									{
									recog.base.set_state(606);
									recog.base.match_token(COMMA,&mut recog.err_handler)?;

									/*InvokeRule identifier*/
									recog.base.set_state(607);
									recog.identifier()?;

									}
									}
									recog.base.set_state(612);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								recog.base.set_state(613);
								recog.base.match_token(RBRACK,&mut recog.err_handler)?;

								}
							}

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(637); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(619);
						recog.base.match_token(ARROW,&mut recog.err_handler)?;

						recog.base.set_state(635);
						recog.err_handler.sync(&mut recog.base)?;
						match recog.base.input.la(1) {
						 IDENT 
							=> {
								{
								/*InvokeRule identifier*/
								recog.base.set_state(620);
								recog.identifier()?;

								}
							}

						 P_OP_ONE 
							=> {
								{
								/*InvokeRule pathOp*/
								recog.base.set_state(621);
								recog.pathOp()?;

								recog.base.set_state(633);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if _la==LBRACK {
									{
									recog.base.set_state(622);
									recog.base.match_token(LBRACK,&mut recog.err_handler)?;

									/*InvokeRule identifier*/
									recog.base.set_state(623);
									recog.identifier()?;

									recog.base.set_state(628);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
									while _la==COMMA {
										{
										{
										recog.base.set_state(624);
										recog.base.match_token(COMMA,&mut recog.err_handler)?;

										/*InvokeRule identifier*/
										recog.base.set_state(625);
										recog.identifier()?;

										}
										}
										recog.base.set_state(630);
										recog.err_handler.sync(&mut recog.base)?;
										_la = recog.base.input.la(1);
									}
									recog.base.set_state(631);
									recog.base.match_token(RBRACK,&mut recog.err_handler)?;

									}
								}

								}
							}

							_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						}
						}
						recog.base.set_state(639); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==ARROW) {break}
					}
					recog.base.set_state(641);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(652);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
						x if x == 1=>{
							{
							{
							recog.base.set_state(642);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							recog.base.set_state(643);
							recog.base.match_token(LBRACE,&mut recog.err_handler)?;

							/*InvokeRule intLiteral*/
							recog.base.set_state(644);
							recog.intLiteral()?;

							}
							recog.base.set_state(648);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==COLON {
								{
								recog.base.set_state(646);
								recog.base.match_token(COLON,&mut recog.err_handler)?;

								/*InvokeRule intLiteral*/
								recog.base.set_state(647);
								recog.intLiteral()?;

								}
							}

							recog.base.set_state(650);
							recog.base.match_token(RBRACE,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pathOp ----------------
pub type PathOpContextAll<'input> = PathOpContext<'input>;


pub type PathOpContext<'input> = BaseParserRuleContext<'input,PathOpContextExt<'input>>;

#[derive(Clone)]
pub struct PathOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PathOpContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PathOpContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pathOp(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_pathOp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PathOpContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_pathOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for PathOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pathOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pathOp }
}
antlr_rust::tid!{PathOpContextExt<'a>}

impl<'input> PathOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PathOpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PathOpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PathOpContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PathOpContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token P_OP_ONE
/// Returns `None` if there is no child corresponding to token P_OP_ONE
fn P_OP_ONE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(P_OP_ONE, 0)
}

}

impl<'input> PathOpContextAttrs<'input> for PathOpContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pathOp(&mut self,)
	-> Result<Rc<PathOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PathOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_pathOp);
        let mut _localctx: Rc<PathOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(656);
			recog.base.match_token(P_OP_ONE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- label ----------------
pub type LabelContextAll<'input> = LabelContext<'input>;


pub type LabelContext<'input> = BaseParserRuleContext<'input,LabelContextExt<'input>>;

#[derive(Clone)]
pub struct LabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for LabelContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for LabelContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_label(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_label(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for LabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_label(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label }
}
antlr_rust::tid!{LabelContextExt<'a>}

impl<'input> LabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<LabelContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRINGLITERAL
/// Returns `None` if there is no child corresponding to token STRINGLITERAL
fn STRINGLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STRINGLITERAL, 0)
}

}

impl<'input> LabelContextAttrs<'input> for LabelContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label(&mut self,)
	-> Result<Rc<LabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_label);
        let mut _localctx: Rc<LabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(658);
			recog.base.match_token(STRINGLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateModifier ----------------
pub type StateModifierContextAll<'input> = StateModifierContext<'input>;


pub type StateModifierContext<'input> = BaseParserRuleContext<'input,StateModifierContextExt<'input>>;

#[derive(Clone)]
pub struct StateModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StateModifierContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StateModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stateModifier(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stateModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StateModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stateModifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateModifier }
}
antlr_rust::tid!{StateModifierContextExt<'a>}

impl<'input> StateModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateModifierContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StateModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token START
/// Returns `None` if there is no child corresponding to token START
fn START(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(START, 0)
}
/// Retrieves first TerminalNode corresponding to token FINAL
/// Returns `None` if there is no child corresponding to token FINAL
fn FINAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(FINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token ABSTRACT
/// Returns `None` if there is no child corresponding to token ABSTRACT
fn ABSTRACT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ABSTRACT, 0)
}
/// Retrieves first TerminalNode corresponding to token NORMAL
/// Returns `None` if there is no child corresponding to token NORMAL
fn NORMAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NORMAL, 0)
}

}

impl<'input> StateModifierContextAttrs<'input> for StateModifierContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateModifier(&mut self,)
	-> Result<Rc<StateModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_stateModifier);
        let mut _localctx: Rc<StateModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(660);
			_la = recog.base.input.la(1);
			if { !(((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & ((1usize << (START - 80)) | (1usize << (FINAL - 80)) | (1usize << (ABSTRACT - 80)) | (1usize << (NORMAL - 80)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for LiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

fn intLiteral(&self) -> Option<Rc<IntLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn realLiteral(&self) -> Option<Rc<RealLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn boolLiteral(&self) -> Option<Rc<BoolLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn charLiteral(&self) -> Option<Rc<CharLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumLiteral(&self) -> Option<Rc<EnumLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bvLiteral(&self) -> Option<Rc<BvLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(669);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule intLiteral*/
					recog.base.set_state(662);
					recog.intLiteral()?;

					}
				}

			 REALLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule realLiteral*/
					recog.base.set_state(663);
					recog.realLiteral()?;

					}
				}

			 BOOLLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule boolLiteral*/
					recog.base.set_state(664);
					recog.boolLiteral()?;

					}
				}

			 STRINGLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(665);
					recog.stringLiteral()?;

					}
				}

			 CHARLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule charLiteral*/
					recog.base.set_state(666);
					recog.charLiteral()?;

					}
				}

			 ENUMLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule enumLiteral*/
					recog.base.set_state(667);
					recog.enumLiteral()?;

					}
				}

			 BVLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule bvLiteral*/
					recog.base.set_state(668);
					recog.bvLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- intLiteral ----------------
pub type IntLiteralContextAll<'input> = IntLiteralContext<'input>;


pub type IntLiteralContext<'input> = BaseParserRuleContext<'input,IntLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct IntLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for IntLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for IntLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_intLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_intLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for IntLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_intLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intLiteral }
}
antlr_rust::tid!{IntLiteralContextExt<'a>}

impl<'input> IntLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IntLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IntLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<IntLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTLITERAL
/// Returns `None` if there is no child corresponding to token INTLITERAL
fn INTLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(INTLITERAL, 0)
}

}

impl<'input> IntLiteralContextAttrs<'input> for IntLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn intLiteral(&mut self,)
	-> Result<Rc<IntLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_intLiteral);
        let mut _localctx: Rc<IntLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(671);
			recog.base.match_token(INTLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- realLiteral ----------------
pub type RealLiteralContextAll<'input> = RealLiteralContext<'input>;


pub type RealLiteralContext<'input> = BaseParserRuleContext<'input,RealLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct RealLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RealLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RealLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_realLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_realLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RealLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_realLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for RealLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_realLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_realLiteral }
}
antlr_rust::tid!{RealLiteralContextExt<'a>}

impl<'input> RealLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RealLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RealLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RealLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RealLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REALLITERAL
/// Returns `None` if there is no child corresponding to token REALLITERAL
fn REALLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(REALLITERAL, 0)
}

}

impl<'input> RealLiteralContextAttrs<'input> for RealLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn realLiteral(&mut self,)
	-> Result<Rc<RealLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RealLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_realLiteral);
        let mut _localctx: Rc<RealLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(673);
			recog.base.match_token(REALLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- boolLiteral ----------------
pub type BoolLiteralContextAll<'input> = BoolLiteralContext<'input>;


pub type BoolLiteralContext<'input> = BaseParserRuleContext<'input,BoolLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct BoolLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BoolLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BoolLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_boolLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_boolLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BoolLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_boolLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoolLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_boolLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_boolLiteral }
}
antlr_rust::tid!{BoolLiteralContextExt<'a>}

impl<'input> BoolLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BoolLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BoolLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BoolLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BoolLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BOOLLITERAL
/// Returns `None` if there is no child corresponding to token BOOLLITERAL
fn BOOLLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BOOLLITERAL, 0)
}

}

impl<'input> BoolLiteralContextAttrs<'input> for BoolLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn boolLiteral(&mut self,)
	-> Result<Rc<BoolLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BoolLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_boolLiteral);
        let mut _localctx: Rc<BoolLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(675);
			recog.base.match_token(BOOLLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stringLiteral ----------------
pub type StringLiteralContextAll<'input> = StringLiteralContext<'input>;


pub type StringLiteralContext<'input> = BaseParserRuleContext<'input,StringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StringLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_stringLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StringLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_stringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLiteral }
}
antlr_rust::tid!{StringLiteralContextExt<'a>}

impl<'input> StringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StringLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRINGLITERAL
/// Returns `None` if there is no child corresponding to token STRINGLITERAL
fn STRINGLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STRINGLITERAL, 0)
}

}

impl<'input> StringLiteralContextAttrs<'input> for StringLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringLiteral(&mut self,)
	-> Result<Rc<StringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_stringLiteral);
        let mut _localctx: Rc<StringLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(677);
			recog.base.match_token(STRINGLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- charLiteral ----------------
pub type CharLiteralContextAll<'input> = CharLiteralContext<'input>;


pub type CharLiteralContext<'input> = BaseParserRuleContext<'input,CharLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct CharLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for CharLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for CharLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_charLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_charLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for CharLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_charLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_charLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_charLiteral }
}
antlr_rust::tid!{CharLiteralContextExt<'a>}

impl<'input> CharLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<CharLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CHARLITERAL
/// Returns `None` if there is no child corresponding to token CHARLITERAL
fn CHARLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(CHARLITERAL, 0)
}

}

impl<'input> CharLiteralContextAttrs<'input> for CharLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn charLiteral(&mut self,)
	-> Result<Rc<CharLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_charLiteral);
        let mut _localctx: Rc<CharLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(679);
			recog.base.match_token(CHARLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bvLiteral ----------------
pub type BvLiteralContextAll<'input> = BvLiteralContext<'input>;


pub type BvLiteralContext<'input> = BaseParserRuleContext<'input,BvLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct BvLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BvLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BvLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bvLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_bvLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BvLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_bvLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for BvLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bvLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bvLiteral }
}
antlr_rust::tid!{BvLiteralContextExt<'a>}

impl<'input> BvLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BvLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BvLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BvLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BvLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BVLITERAL
/// Returns `None` if there is no child corresponding to token BVLITERAL
fn BVLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BVLITERAL, 0)
}

}

impl<'input> BvLiteralContextAttrs<'input> for BvLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bvLiteral(&mut self,)
	-> Result<Rc<BvLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BvLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_bvLiteral);
        let mut _localctx: Rc<BvLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(681);
			recog.base.match_token(BVLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumLiteral ----------------
pub type EnumLiteralContextAll<'input> = EnumLiteralContext<'input>;


pub type EnumLiteralContext<'input> = BaseParserRuleContext<'input,EnumLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for EnumLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for EnumLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_enumLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for EnumLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_enumLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumLiteral }
}
antlr_rust::tid!{EnumLiteralContextExt<'a>}

impl<'input> EnumLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumLiteralContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<EnumLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENUMLITERAL
/// Returns `None` if there is no child corresponding to token ENUMLITERAL
fn ENUMLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ENUMLITERAL, 0)
}

}

impl<'input> EnumLiteralContextAttrs<'input> for EnumLiteralContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumLiteral(&mut self,)
	-> Result<Rc<EnumLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_enumLiteral);
        let mut _localctx: Rc<EnumLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(683);
			recog.base.match_token(ENUMLITERAL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- record ----------------
pub type RecordContextAll<'input> = RecordContext<'input>;


pub type RecordContext<'input> = BaseParserRuleContext<'input,RecordContextExt<'input>>;

#[derive(Clone)]
pub struct RecordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RecordContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RecordContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_record(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_record(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RecordContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_record(self);
	}
}

impl<'input> CustomRuleContext<'input> for RecordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_record }
	//fn type_rule_index() -> usize where Self: Sized { RULE_record }
}
antlr_rust::tid!{RecordContextExt<'a>}

impl<'input> RecordContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RecordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RECORD
/// Returns `None` if there is no child corresponding to token RECORD
fn RECORD(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RECORD, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordScope(&self) -> Option<Rc<RecordScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> RecordContextAttrs<'input> for RecordContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn record(&mut self,)
	-> Result<Rc<RecordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_record);
        let mut _localctx: Rc<RecordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(685);
			recog.base.match_token(RECORD,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(686);
			recog.identifier()?;

			/*InvokeRule recordScope*/
			recog.base.set_state(687);
			recog.recordScope()?;

			recog.base.set_state(688);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordScope ----------------
pub type RecordScopeContextAll<'input> = RecordScopeContext<'input>;


pub type RecordScopeContext<'input> = BaseParserRuleContext<'input,RecordScopeContextExt<'input>>;

#[derive(Clone)]
pub struct RecordScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RecordScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RecordScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_recordScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RecordScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_recordScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for RecordScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordScope }
}
antlr_rust::tid!{RecordScopeContextExt<'a>}

impl<'input> RecordScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RecordScopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn recordVariableDeclGroup(&self) -> Option<Rc<RecordVariableDeclGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}

}

impl<'input> RecordScopeContextAttrs<'input> for RecordScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordScope(&mut self,)
	-> Result<Rc<RecordScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_recordScope);
        let mut _localctx: Rc<RecordScopeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(690);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule recordVariableDeclGroup*/
			recog.base.set_state(691);
			recog.recordVariableDeclGroup()?;

			recog.base.set_state(692);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordVariableDeclGroup ----------------
pub type RecordVariableDeclGroupContextAll<'input> = RecordVariableDeclGroupContext<'input>;


pub type RecordVariableDeclGroupContext<'input> = BaseParserRuleContext<'input,RecordVariableDeclGroupContextExt<'input>>;

#[derive(Clone)]
pub struct RecordVariableDeclGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RecordVariableDeclGroupContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RecordVariableDeclGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordVariableDeclGroup(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_recordVariableDeclGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RecordVariableDeclGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_recordVariableDeclGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for RecordVariableDeclGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordVariableDeclGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordVariableDeclGroup }
}
antlr_rust::tid!{RecordVariableDeclGroupContextExt<'a>}

impl<'input> RecordVariableDeclGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordVariableDeclGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordVariableDeclGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordVariableDeclGroupContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RecordVariableDeclGroupContextExt<'input>>{

fn recordVariableDecl_all(&self) ->  Vec<Rc<RecordVariableDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn recordVariableDecl(&self, i: usize) -> Option<Rc<RecordVariableDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RecordVariableDeclGroupContextAttrs<'input> for RecordVariableDeclGroupContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordVariableDeclGroup(&mut self,)
	-> Result<Rc<RecordVariableDeclGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordVariableDeclGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_recordVariableDeclGroup);
        let mut _localctx: Rc<RecordVariableDeclGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(695); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule recordVariableDecl*/
				recog.base.set_state(694);
				recog.recordVariableDecl()?;

				}
				}
				recog.base.set_state(697); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (INT - 73)) | (1usize << (BOOL - 73)) | (1usize << (REAL - 73)) | (1usize << (STRING - 73)) | (1usize << (ENUM - 73)))) != 0) || _la==BV) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordVariableDecl ----------------
pub type RecordVariableDeclContextAll<'input> = RecordVariableDeclContext<'input>;


pub type RecordVariableDeclContext<'input> = BaseParserRuleContext<'input,RecordVariableDeclContextExt<'input>>;

#[derive(Clone)]
pub struct RecordVariableDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RecordVariableDeclContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RecordVariableDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordVariableDecl(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_recordVariableDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RecordVariableDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_recordVariableDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for RecordVariableDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordVariableDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordVariableDecl }
}
antlr_rust::tid!{RecordVariableDeclContextExt<'a>}

impl<'input> RecordVariableDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordVariableDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordVariableDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordVariableDeclContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RecordVariableDeclContextExt<'input>>{

fn type_mark(&self) -> Option<Rc<Type_markContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarator(&self) -> Option<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> RecordVariableDeclContextAttrs<'input> for RecordVariableDeclContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordVariableDecl(&mut self,)
	-> Result<Rc<RecordVariableDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordVariableDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_recordVariableDecl);
        let mut _localctx: Rc<RecordVariableDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_mark*/
			recog.base.set_state(699);
			recog.type_mark()?;

			/*InvokeRule variableDeclarator*/
			recog.base.set_state(700);
			recog.variableDeclarator()?;

			recog.base.set_state(701);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- globalConstantGroup ----------------
pub type GlobalConstantGroupContextAll<'input> = GlobalConstantGroupContext<'input>;


pub type GlobalConstantGroupContext<'input> = BaseParserRuleContext<'input,GlobalConstantGroupContextExt<'input>>;

#[derive(Clone)]
pub struct GlobalConstantGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for GlobalConstantGroupContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for GlobalConstantGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_globalConstantGroup(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_globalConstantGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for GlobalConstantGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_globalConstantGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for GlobalConstantGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_globalConstantGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_globalConstantGroup }
}
antlr_rust::tid!{GlobalConstantGroupContextExt<'a>}

impl<'input> GlobalConstantGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GlobalConstantGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GlobalConstantGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GlobalConstantGroupContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<GlobalConstantGroupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONST
/// Returns `None` if there is no child corresponding to token CONST
fn CONST(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(CONST, 0)
}
fn type_mark(&self) -> Option<Rc<Type_markContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn globalConstantDecl_all(&self) ->  Vec<Rc<GlobalConstantDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn globalConstantDecl(&self, i: usize) -> Option<Rc<GlobalConstantDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> GlobalConstantGroupContextAttrs<'input> for GlobalConstantGroupContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn globalConstantGroup(&mut self,)
	-> Result<Rc<GlobalConstantGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GlobalConstantGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_globalConstantGroup);
        let mut _localctx: Rc<GlobalConstantGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(703);
			recog.base.match_token(CONST,&mut recog.err_handler)?;

			/*InvokeRule type_mark*/
			recog.base.set_state(704);
			recog.type_mark()?;

			/*InvokeRule globalConstantDecl*/
			recog.base.set_state(705);
			recog.globalConstantDecl()?;

			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(706);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule globalConstantDecl*/
				recog.base.set_state(707);
				recog.globalConstantDecl()?;

				}
				}
				recog.base.set_state(712);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(713);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- globalConstantDecl ----------------
pub type GlobalConstantDeclContextAll<'input> = GlobalConstantDeclContext<'input>;


pub type GlobalConstantDeclContext<'input> = BaseParserRuleContext<'input,GlobalConstantDeclContextExt<'input>>;

#[derive(Clone)]
pub struct GlobalConstantDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for GlobalConstantDeclContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for GlobalConstantDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_globalConstantDecl(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_globalConstantDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for GlobalConstantDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_globalConstantDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for GlobalConstantDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_globalConstantDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_globalConstantDecl }
}
antlr_rust::tid!{GlobalConstantDeclContextExt<'a>}

impl<'input> GlobalConstantDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GlobalConstantDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GlobalConstantDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GlobalConstantDeclContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<GlobalConstantDeclContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
fn variableInitializer(&self) -> Option<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GlobalConstantDeclContextAttrs<'input> for GlobalConstantDeclContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn globalConstantDecl(&mut self,)
	-> Result<Rc<GlobalConstantDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GlobalConstantDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_globalConstantDecl);
        let mut _localctx: Rc<GlobalConstantDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(715);
			recog.identifier()?;

			recog.base.set_state(716);
			recog.base.match_token(EQUAL,&mut recog.err_handler)?;

			/*InvokeRule variableInitializer*/
			recog.base.set_state(717);
			recog.variableInitializer()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- globalVariableGroup ----------------
pub type GlobalVariableGroupContextAll<'input> = GlobalVariableGroupContext<'input>;


pub type GlobalVariableGroupContext<'input> = BaseParserRuleContext<'input,GlobalVariableGroupContextExt<'input>>;

#[derive(Clone)]
pub struct GlobalVariableGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for GlobalVariableGroupContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for GlobalVariableGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_globalVariableGroup(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_globalVariableGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for GlobalVariableGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_globalVariableGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for GlobalVariableGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_globalVariableGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_globalVariableGroup }
}
antlr_rust::tid!{GlobalVariableGroupContextExt<'a>}

impl<'input> GlobalVariableGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GlobalVariableGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GlobalVariableGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GlobalVariableGroupContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<GlobalVariableGroupContextExt<'input>>{

fn type_mark(&self) -> Option<Rc<Type_markContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarator_all(&self) ->  Vec<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableDeclarator(&self, i: usize) -> Option<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> GlobalVariableGroupContextAttrs<'input> for GlobalVariableGroupContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn globalVariableGroup(&mut self,)
	-> Result<Rc<GlobalVariableGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GlobalVariableGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_globalVariableGroup);
        let mut _localctx: Rc<GlobalVariableGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_mark*/
			recog.base.set_state(719);
			recog.type_mark()?;

			/*InvokeRule variableDeclarator*/
			recog.base.set_state(720);
			recog.variableDeclarator()?;

			recog.base.set_state(725);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(721);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule variableDeclarator*/
				recog.base.set_state(722);
				recog.variableDeclarator()?;

				}
				}
				recog.base.set_state(727);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(728);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- localVariableGroup ----------------
pub type LocalVariableGroupContextAll<'input> = LocalVariableGroupContext<'input>;


pub type LocalVariableGroupContext<'input> = BaseParserRuleContext<'input,LocalVariableGroupContextExt<'input>>;

#[derive(Clone)]
pub struct LocalVariableGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for LocalVariableGroupContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for LocalVariableGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_localVariableGroup(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_localVariableGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for LocalVariableGroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_localVariableGroup(self);
	}
}

impl<'input> CustomRuleContext<'input> for LocalVariableGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVariableGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVariableGroup }
}
antlr_rust::tid!{LocalVariableGroupContextExt<'a>}

impl<'input> LocalVariableGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LocalVariableGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LocalVariableGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LocalVariableGroupContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<LocalVariableGroupContextExt<'input>>{

fn type_mark(&self) -> Option<Rc<Type_markContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarator_all(&self) ->  Vec<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableDeclarator(&self, i: usize) -> Option<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> LocalVariableGroupContextAttrs<'input> for LocalVariableGroupContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn localVariableGroup(&mut self,)
	-> Result<Rc<LocalVariableGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LocalVariableGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_localVariableGroup);
        let mut _localctx: Rc<LocalVariableGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_mark*/
			recog.base.set_state(730);
			recog.type_mark()?;

			/*InvokeRule variableDeclarator*/
			recog.base.set_state(731);
			recog.variableDeclarator()?;

			recog.base.set_state(736);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(732);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule variableDeclarator*/
				recog.base.set_state(733);
				recog.variableDeclarator()?;

				}
				}
				recog.base.set_state(738);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(739);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modifier ----------------
pub type ModifierContextAll<'input> = ModifierContext<'input>;


pub type ModifierContext<'input> = BaseParserRuleContext<'input,ModifierContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ModifierContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modifier(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_modifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ModifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_modifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifier }
}
antlr_rust::tid!{ModifierContextExt<'a>}

impl<'input> ModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GLOBAL
/// Returns `None` if there is no child corresponding to token GLOBAL
fn GLOBAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(GLOBAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NATIVE
/// Returns `None` if there is no child corresponding to token NATIVE
fn NATIVE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NATIVE, 0)
}

}

impl<'input> ModifierContextAttrs<'input> for ModifierContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifier(&mut self,)
	-> Result<Rc<ModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_modifier);
        let mut _localctx: Rc<ModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(741);
			_la = recog.base.input.la(1);
			if { !(_la==GLOBAL || _la==NATIVE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_mark ----------------
pub type Type_markContextAll<'input> = Type_markContext<'input>;


pub type Type_markContext<'input> = BaseParserRuleContext<'input,Type_markContextExt<'input>>;

#[derive(Clone)]
pub struct Type_markContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for Type_markContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for Type_markContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_mark(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_type_mark(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for Type_markContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_type_mark(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_markContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_mark }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_mark }
}
antlr_rust::tid!{Type_markContextExt<'a>}

impl<'input> Type_markContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_markContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_markContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_markContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<Type_markContextExt<'input>>{

fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumType(&self) -> Option<Rc<EnumTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bvType(&self) -> Option<Rc<BvTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_markContextAttrs<'input> for Type_markContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_mark(&mut self,)
	-> Result<Rc<Type_markContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_markContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_type_mark);
        let mut _localctx: Rc<Type_markContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(746);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT | BOOL | REAL | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule primitiveType*/
					recog.base.set_state(743);
					recog.primitiveType()?;

					}
				}

			 ENUM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule enumType*/
					recog.base.set_state(744);
					recog.enumType()?;

					}
				}

			 BV 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule bvType*/
					recog.base.set_state(745);
					recog.bvType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primitiveBvType ----------------
pub type PrimitiveBvTypeContextAll<'input> = PrimitiveBvTypeContext<'input>;


pub type PrimitiveBvTypeContext<'input> = BaseParserRuleContext<'input,PrimitiveBvTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrimitiveBvTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PrimitiveBvTypeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PrimitiveBvTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primitiveBvType(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_primitiveBvType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PrimitiveBvTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_primitiveBvType(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimitiveBvTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primitiveBvType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primitiveBvType }
}
antlr_rust::tid!{PrimitiveBvTypeContextExt<'a>}

impl<'input> PrimitiveBvTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimitiveBvTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimitiveBvTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimitiveBvTypeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PrimitiveBvTypeContextExt<'input>>{

fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bvType(&self) -> Option<Rc<BvTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimitiveBvTypeContextAttrs<'input> for PrimitiveBvTypeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primitiveBvType(&mut self,)
	-> Result<Rc<PrimitiveBvTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimitiveBvTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_primitiveBvType);
        let mut _localctx: Rc<PrimitiveBvTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(750);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT | BOOL | REAL | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule primitiveType*/
					recog.base.set_state(748);
					recog.primitiveType()?;

					}
				}

			 BV 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule bvType*/
					recog.base.set_state(749);
					recog.bvType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bvType ----------------
pub type BvTypeContextAll<'input> = BvTypeContext<'input>;


pub type BvTypeContext<'input> = BaseParserRuleContext<'input,BvTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BvTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BvTypeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BvTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bvType(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_bvType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BvTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_bvType(self);
	}
}

impl<'input> CustomRuleContext<'input> for BvTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bvType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bvType }
}
antlr_rust::tid!{BvTypeContextExt<'a>}

impl<'input> BvTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BvTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BvTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BvTypeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BvTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BV
/// Returns `None` if there is no child corresponding to token BV
fn BV(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BV, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token INTLITERAL
/// Returns `None` if there is no child corresponding to token INTLITERAL
fn INTLITERAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(INTLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}

}

impl<'input> BvTypeContextAttrs<'input> for BvTypeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bvType(&mut self,)
	-> Result<Rc<BvTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BvTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_bvType);
        let mut _localctx: Rc<BvTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(752);
			recog.base.match_token(BV,&mut recog.err_handler)?;

			recog.base.set_state(753);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			recog.base.set_state(754);
			recog.base.match_token(INTLITERAL,&mut recog.err_handler)?;

			recog.base.set_state(755);
			recog.base.match_token(RBRACK,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primitiveType ----------------
pub type PrimitiveTypeContextAll<'input> = PrimitiveTypeContext<'input>;


pub type PrimitiveTypeContext<'input> = BaseParserRuleContext<'input,PrimitiveTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrimitiveTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PrimitiveTypeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PrimitiveTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primitiveType(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_primitiveType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PrimitiveTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_primitiveType(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimitiveTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primitiveType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primitiveType }
}
antlr_rust::tid!{PrimitiveTypeContextExt<'a>}

impl<'input> PrimitiveTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimitiveTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimitiveTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimitiveTypeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PrimitiveTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}
/// Retrieves first TerminalNode corresponding to token REAL
/// Returns `None` if there is no child corresponding to token REAL
fn REAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(REAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> PrimitiveTypeContextAttrs<'input> for PrimitiveTypeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primitiveType(&mut self,)
	-> Result<Rc<PrimitiveTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimitiveTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_primitiveType);
        let mut _localctx: Rc<PrimitiveTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(757);
			_la = recog.base.input.la(1);
			if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (INT - 73)) | (1usize << (BOOL - 73)) | (1usize << (REAL - 73)) | (1usize << (STRING - 73)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumType ----------------
pub type EnumTypeContextAll<'input> = EnumTypeContext<'input>;


pub type EnumTypeContext<'input> = BaseParserRuleContext<'input,EnumTypeContextExt<'input>>;

#[derive(Clone)]
pub struct EnumTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for EnumTypeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for EnumTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumType(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_enumType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for EnumTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_enumType(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumType }
}
antlr_rust::tid!{EnumTypeContextExt<'a>}

impl<'input> EnumTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumTypeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<EnumTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENUM
/// Returns `None` if there is no child corresponding to token ENUM
fn ENUM(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ENUM, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn enumDecl_all(&self) ->  Vec<Rc<EnumDeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumDecl(&self, i: usize) -> Option<Rc<EnumDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> EnumTypeContextAttrs<'input> for EnumTypeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumType(&mut self,)
	-> Result<Rc<EnumTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_enumType);
        let mut _localctx: Rc<EnumTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(759);
			recog.base.match_token(ENUM,&mut recog.err_handler)?;

			recog.base.set_state(760);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule enumDecl*/
			recog.base.set_state(761);
			recog.enumDecl()?;

			recog.base.set_state(766);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(762);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule enumDecl*/
				recog.base.set_state(763);
				recog.enumDecl()?;

				}
				}
				recog.base.set_state(768);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(769);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumDecl ----------------
pub type EnumDeclContextAll<'input> = EnumDeclContext<'input>;


pub type EnumDeclContext<'input> = BaseParserRuleContext<'input,EnumDeclContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for EnumDeclContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for EnumDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumDecl(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_enumDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for EnumDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_enumDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumDecl }
}
antlr_rust::tid!{EnumDeclContextExt<'a>}

impl<'input> EnumDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumDeclContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<EnumDeclContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumDeclContextAttrs<'input> for EnumDeclContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumDecl(&mut self,)
	-> Result<Rc<EnumDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_enumDecl);
        let mut _localctx: Rc<EnumDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(771);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclarator ----------------
pub type VariableDeclaratorContextAll<'input> = VariableDeclaratorContext<'input>;


pub type VariableDeclaratorContext<'input> = BaseParserRuleContext<'input,VariableDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for VariableDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for VariableDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableDeclarator(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_variableDeclarator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for VariableDeclaratorContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_variableDeclarator(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclarator }
}
antlr_rust::tid!{VariableDeclaratorContextExt<'a>}

impl<'input> VariableDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclaratorContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<VariableDeclaratorContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
fn variableInitializer(&self) -> Option<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whereExpr(&self) -> Option<Rc<WhereExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableDeclaratorContextAttrs<'input> for VariableDeclaratorContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclarator(&mut self,)
	-> Result<Rc<VariableDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_variableDeclarator);
        let mut _localctx: Rc<VariableDeclaratorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(773);
			recog.identifier()?;

			recog.base.set_state(776);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EQUAL {
				{
				recog.base.set_state(774);
				recog.base.match_token(EQUAL,&mut recog.err_handler)?;

				/*InvokeRule variableInitializer*/
				recog.base.set_state(775);
				recog.variableInitializer()?;

				}
			}

			recog.base.set_state(779);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WHERE {
				{
				/*InvokeRule whereExpr*/
				recog.base.set_state(778);
				recog.whereExpr()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- whereExpr ----------------
pub type WhereExprContextAll<'input> = WhereExprContext<'input>;


pub type WhereExprContext<'input> = BaseParserRuleContext<'input,WhereExprContextExt<'input>>;

#[derive(Clone)]
pub struct WhereExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for WhereExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for WhereExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_whereExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_whereExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for WhereExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_whereExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhereExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whereExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whereExpr }
}
antlr_rust::tid!{WhereExprContextExt<'a>}

impl<'input> WhereExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhereExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhereExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhereExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<WhereExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(WHERE, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WhereExprContextAttrs<'input> for WhereExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whereExpr(&mut self,)
	-> Result<Rc<WhereExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhereExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_whereExpr);
        let mut _localctx: Rc<WhereExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(781);
			recog.base.match_token(WHERE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(782);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableInitializer ----------------
pub type VariableInitializerContextAll<'input> = VariableInitializerContext<'input>;


pub type VariableInitializerContext<'input> = BaseParserRuleContext<'input,VariableInitializerContextExt<'input>>;

#[derive(Clone)]
pub struct VariableInitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for VariableInitializerContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for VariableInitializerContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableInitializer(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_variableInitializer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for VariableInitializerContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_variableInitializer(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableInitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableInitializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableInitializer }
}
antlr_rust::tid!{VariableInitializerContextExt<'a>}

impl<'input> VariableInitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableInitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableInitializerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableInitializerContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<VariableInitializerContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableInitializerContextAttrs<'input> for VariableInitializerContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableInitializer(&mut self,)
	-> Result<Rc<VariableInitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableInitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_variableInitializer);
        let mut _localctx: Rc<VariableInitializerContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(784);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assertExpr ----------------
pub type AssertExprContextAll<'input> = AssertExprContext<'input>;


pub type AssertExprContext<'input> = BaseParserRuleContext<'input,AssertExprContextExt<'input>>;

#[derive(Clone)]
pub struct AssertExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for AssertExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for AssertExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assertExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_assertExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for AssertExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_assertExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssertExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assertExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assertExpr }
}
antlr_rust::tid!{AssertExprContextExt<'a>}

impl<'input> AssertExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssertExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssertExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssertExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<AssertExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ASSERT
/// Returns `None` if there is no child corresponding to token ASSERT
fn ASSERT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSERT, 0)
}
fn assertMainExpr(&self) -> Option<Rc<AssertMainExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn annotationExpr(&self) -> Option<Rc<AnnotationExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inExpr(&self) -> Option<Rc<InExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssertExprContextAttrs<'input> for AssertExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assertExpr(&mut self,)
	-> Result<Rc<AssertExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssertExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_assertExpr);
        let mut _localctx: Rc<AssertExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(787);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==AT_SIGN {
				{
				/*InvokeRule annotationExpr*/
				recog.base.set_state(786);
				recog.annotationExpr()?;

				}
			}

			recog.base.set_state(789);
			recog.base.match_token(ASSERT,&mut recog.err_handler)?;

			/*InvokeRule assertMainExpr*/
			recog.base.set_state(790);
			recog.assertMainExpr()?;

			recog.base.set_state(792);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IN {
				{
				/*InvokeRule inExpr*/
				recog.base.set_state(791);
				recog.inExpr()?;

				}
			}

			recog.base.set_state(794);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assertMainExpr ----------------
pub type AssertMainExprContextAll<'input> = AssertMainExprContext<'input>;


pub type AssertMainExprContext<'input> = BaseParserRuleContext<'input,AssertMainExprContextExt<'input>>;

#[derive(Clone)]
pub struct AssertMainExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for AssertMainExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for AssertMainExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assertMainExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_assertMainExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for AssertMainExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_assertMainExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssertMainExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assertMainExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assertMainExpr }
}
antlr_rust::tid!{AssertMainExprContextExt<'a>}

impl<'input> AssertMainExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssertMainExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssertMainExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssertMainExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<AssertMainExprContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ALWAYS
/// Returns `None` if there is no child corresponding to token ALWAYS
fn ALWAYS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ALWAYS, 0)
}
/// Retrieves first TerminalNode corresponding to token SOME
/// Returns `None` if there is no child corresponding to token SOME
fn SOME(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SOME, 0)
}

}

impl<'input> AssertMainExprContextAttrs<'input> for AssertMainExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assertMainExpr(&mut self,)
	-> Result<Rc<AssertMainExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssertMainExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_assertMainExpr);
        let mut _localctx: Rc<AssertMainExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(797);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ALWAYS || _la==SOME {
				{
				recog.base.set_state(796);
				_la = recog.base.input.la(1);
				if { !(_la==ALWAYS || _la==SOME) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			/*InvokeRule expression*/
			recog.base.set_state(799);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(801);
			recog.expression()?;

			recog.base.set_state(802);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn conditionalImpliesExpression(&self) -> Option<Rc<ConditionalImpliesExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_PLUS_EQ
/// Returns `None` if there is no child corresponding to token ASSIGN_PLUS_EQ
fn ASSIGN_PLUS_EQ(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_PLUS_EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_MINUS_EQ
/// Returns `None` if there is no child corresponding to token ASSIGN_MINUS_EQ
fn ASSIGN_MINUS_EQ(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_MINUS_EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_TIMES_EQ
/// Returns `None` if there is no child corresponding to token ASSIGN_TIMES_EQ
fn ASSIGN_TIMES_EQ(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_TIMES_EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_DIV_EQ
/// Returns `None` if there is no child corresponding to token ASSIGN_DIV_EQ
fn ASSIGN_DIV_EQ(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_DIV_EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_SHIFT_LEFT
/// Returns `None` if there is no child corresponding to token ASSIGN_SHIFT_LEFT
fn ASSIGN_SHIFT_LEFT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_SHIFT_LEFT, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN_SHIFT_RIGHT
/// Returns `None` if there is no child corresponding to token ASSIGN_SHIFT_RIGHT
fn ASSIGN_SHIFT_RIGHT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN_SHIFT_RIGHT, 0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalImpliesExpression*/
			recog.base.set_state(804);
			recog.conditionalImpliesExpression()?;

			recog.base.set_state(807);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(82,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(805);
					_la = recog.base.input.la(1);
					if { !(_la==EQUAL || ((((_la - 42)) & !0x3f) == 0 && ((1usize << (_la - 42)) & ((1usize << (ASSIGN_PLUS_EQ - 42)) | (1usize << (ASSIGN_MINUS_EQ - 42)) | (1usize << (ASSIGN_TIMES_EQ - 42)) | (1usize << (ASSIGN_DIV_EQ - 42)) | (1usize << (ASSIGN_SHIFT_LEFT - 42)) | (1usize << (ASSIGN_SHIFT_RIGHT - 42)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(806);
					recog.expression()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- conditionalImpliesExpression ----------------
pub type ConditionalImpliesExpressionContextAll<'input> = ConditionalImpliesExpressionContext<'input>;


pub type ConditionalImpliesExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalImpliesExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalImpliesExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ConditionalImpliesExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ConditionalImpliesExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_conditionalImpliesExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_conditionalImpliesExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ConditionalImpliesExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalImpliesExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalImpliesExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalImpliesExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalImpliesExpression }
}
antlr_rust::tid!{ConditionalImpliesExpressionContextExt<'a>}

impl<'input> ConditionalImpliesExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionalImpliesExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalImpliesExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalImpliesExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ConditionalImpliesExpressionContextExt<'input>>{

fn conditionalOrExpression_all(&self) ->  Vec<Rc<ConditionalOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalOrExpression(&self, i: usize) -> Option<Rc<ConditionalOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token IMPLIES in current rule
fn IMPLIES_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IMPLIES, starting from 0.
/// Returns `None` if number of children corresponding to token IMPLIES is less or equal than `i`.
fn IMPLIES(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(IMPLIES, i)
}

}

impl<'input> ConditionalImpliesExpressionContextAttrs<'input> for ConditionalImpliesExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditionalImpliesExpression(&mut self,)
	-> Result<Rc<ConditionalImpliesExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalImpliesExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_conditionalImpliesExpression);
        let mut _localctx: Rc<ConditionalImpliesExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalOrExpression*/
			recog.base.set_state(809);
			recog.conditionalOrExpression()?;

			recog.base.set_state(814);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(810);
					recog.base.match_token(IMPLIES,&mut recog.err_handler)?;

					/*InvokeRule conditionalOrExpression*/
					recog.base.set_state(811);
					recog.conditionalOrExpression()?;

					}
					} 
				}
				recog.base.set_state(816);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- conditionalOrExpression ----------------
pub type ConditionalOrExpressionContextAll<'input> = ConditionalOrExpressionContext<'input>;


pub type ConditionalOrExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ConditionalOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ConditionalOrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_conditionalOrExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_conditionalOrExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ConditionalOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalOrExpression }
}
antlr_rust::tid!{ConditionalOrExpressionContextExt<'a>}

impl<'input> ConditionalOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionalOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalOrExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalOrExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ConditionalOrExpressionContextExt<'input>>{

fn conditionalAndExpression_all(&self) ->  Vec<Rc<ConditionalAndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalAndExpression(&self, i: usize) -> Option<Rc<ConditionalAndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> ConditionalOrExpressionContextAttrs<'input> for ConditionalOrExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditionalOrExpression(&mut self,)
	-> Result<Rc<ConditionalOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_conditionalOrExpression);
        let mut _localctx: Rc<ConditionalOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalAndExpression*/
			recog.base.set_state(817);
			recog.conditionalAndExpression()?;

			recog.base.set_state(822);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(818);
					recog.base.match_token(OR,&mut recog.err_handler)?;

					/*InvokeRule conditionalAndExpression*/
					recog.base.set_state(819);
					recog.conditionalAndExpression()?;

					}
					} 
				}
				recog.base.set_state(824);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- conditionalAndExpression ----------------
pub type ConditionalAndExpressionContextAll<'input> = ConditionalAndExpressionContext<'input>;


pub type ConditionalAndExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalAndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalAndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ConditionalAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ConditionalAndExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_conditionalAndExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_conditionalAndExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ConditionalAndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalAndExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalAndExpression }
}
antlr_rust::tid!{ConditionalAndExpressionContextExt<'a>}

impl<'input> ConditionalAndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionalAndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalAndExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalAndExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ConditionalAndExpressionContextExt<'input>>{

fn conditionalXorExpression_all(&self) ->  Vec<Rc<ConditionalXorExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalXorExpression(&self, i: usize) -> Option<Rc<ConditionalXorExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}

}

impl<'input> ConditionalAndExpressionContextAttrs<'input> for ConditionalAndExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditionalAndExpression(&mut self,)
	-> Result<Rc<ConditionalAndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalAndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_conditionalAndExpression);
        let mut _localctx: Rc<ConditionalAndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule conditionalXorExpression*/
			recog.base.set_state(825);
			recog.conditionalXorExpression()?;

			recog.base.set_state(830);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(826);
					recog.base.match_token(AND,&mut recog.err_handler)?;

					/*InvokeRule conditionalXorExpression*/
					recog.base.set_state(827);
					recog.conditionalXorExpression()?;

					}
					} 
				}
				recog.base.set_state(832);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- conditionalXorExpression ----------------
pub type ConditionalXorExpressionContextAll<'input> = ConditionalXorExpressionContext<'input>;


pub type ConditionalXorExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalXorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalXorExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ConditionalXorExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ConditionalXorExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_conditionalXorExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_conditionalXorExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ConditionalXorExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalXorExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalXorExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalXorExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalXorExpression }
}
antlr_rust::tid!{ConditionalXorExpressionContextExt<'a>}

impl<'input> ConditionalXorExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConditionalXorExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalXorExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalXorExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ConditionalXorExpressionContextExt<'input>>{

fn bitwiseOrExpression_all(&self) ->  Vec<Rc<BitwiseOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn bitwiseOrExpression(&self, i: usize) -> Option<Rc<BitwiseOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token HAT in current rule
fn HAT_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token HAT, starting from 0.
/// Returns `None` if number of children corresponding to token HAT is less or equal than `i`.
fn HAT(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(HAT, i)
}

}

impl<'input> ConditionalXorExpressionContextAttrs<'input> for ConditionalXorExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditionalXorExpression(&mut self,)
	-> Result<Rc<ConditionalXorExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalXorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_conditionalXorExpression);
        let mut _localctx: Rc<ConditionalXorExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule bitwiseOrExpression*/
			recog.base.set_state(833);
			recog.bitwiseOrExpression()?;

			recog.base.set_state(838);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(834);
					recog.base.match_token(HAT,&mut recog.err_handler)?;

					/*InvokeRule bitwiseOrExpression*/
					recog.base.set_state(835);
					recog.bitwiseOrExpression()?;

					}
					} 
				}
				recog.base.set_state(840);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bitwiseOrExpression ----------------
pub type BitwiseOrExpressionContextAll<'input> = BitwiseOrExpressionContext<'input>;


pub type BitwiseOrExpressionContext<'input> = BaseParserRuleContext<'input,BitwiseOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct BitwiseOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BitwiseOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BitwiseOrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bitwiseOrExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_bitwiseOrExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BitwiseOrExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_bitwiseOrExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitwiseOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bitwiseOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bitwiseOrExpression }
}
antlr_rust::tid!{BitwiseOrExpressionContextExt<'a>}

impl<'input> BitwiseOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BitwiseOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BitwiseOrExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BitwiseOrExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BitwiseOrExpressionContextExt<'input>>{

fn bitwiseAndExpression_all(&self) ->  Vec<Rc<BitwiseAndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn bitwiseAndExpression(&self, i: usize) -> Option<Rc<BitwiseAndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token BAR in current rule
fn BAR_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BAR, starting from 0.
/// Returns `None` if number of children corresponding to token BAR is less or equal than `i`.
fn BAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BAR, i)
}

}

impl<'input> BitwiseOrExpressionContextAttrs<'input> for BitwiseOrExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bitwiseOrExpression(&mut self,)
	-> Result<Rc<BitwiseOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BitwiseOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_bitwiseOrExpression);
        let mut _localctx: Rc<BitwiseOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule bitwiseAndExpression*/
			recog.base.set_state(841);
			recog.bitwiseAndExpression()?;

			recog.base.set_state(846);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(842);
					recog.base.match_token(BAR,&mut recog.err_handler)?;

					/*InvokeRule bitwiseAndExpression*/
					recog.base.set_state(843);
					recog.bitwiseAndExpression()?;

					}
					} 
				}
				recog.base.set_state(848);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bitwiseAndExpression ----------------
pub type BitwiseAndExpressionContextAll<'input> = BitwiseAndExpressionContext<'input>;


pub type BitwiseAndExpressionContext<'input> = BaseParserRuleContext<'input,BitwiseAndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct BitwiseAndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BitwiseAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BitwiseAndExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bitwiseAndExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_bitwiseAndExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BitwiseAndExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_bitwiseAndExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitwiseAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bitwiseAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bitwiseAndExpression }
}
antlr_rust::tid!{BitwiseAndExpressionContextExt<'a>}

impl<'input> BitwiseAndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BitwiseAndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BitwiseAndExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BitwiseAndExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BitwiseAndExpressionContextExt<'input>>{

fn equalityExpression_all(&self) ->  Vec<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equalityExpression(&self, i: usize) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token BIT_AND in current rule
fn BIT_AND_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BIT_AND, starting from 0.
/// Returns `None` if number of children corresponding to token BIT_AND is less or equal than `i`.
fn BIT_AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BIT_AND, i)
}

}

impl<'input> BitwiseAndExpressionContextAttrs<'input> for BitwiseAndExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bitwiseAndExpression(&mut self,)
	-> Result<Rc<BitwiseAndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BitwiseAndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_bitwiseAndExpression);
        let mut _localctx: Rc<BitwiseAndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule equalityExpression*/
			recog.base.set_state(849);
			recog.equalityExpression()?;

			recog.base.set_state(854);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(850);
					recog.base.match_token(BIT_AND,&mut recog.err_handler)?;

					/*InvokeRule equalityExpression*/
					recog.base.set_state(851);
					recog.equalityExpression()?;

					}
					} 
				}
				recog.base.set_state(856);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equalityExpression ----------------
pub type EqualityExpressionContextAll<'input> = EqualityExpressionContext<'input>;


pub type EqualityExpressionContext<'input> = BaseParserRuleContext<'input,EqualityExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for EqualityExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for EqualityExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equalityExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_equalityExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for EqualityExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_equalityExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}
antlr_rust::tid!{EqualityExpressionContextExt<'a>}

impl<'input> EqualityExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<EqualityExpressionContextExt<'input>>{

fn relationalExpression_all(&self) ->  Vec<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relationalExpression(&self, i: usize) -> Option<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOUBLE_EQUAL in current rule
fn DOUBLE_EQUAL_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOUBLE_EQUAL, starting from 0.
/// Returns `None` if number of children corresponding to token DOUBLE_EQUAL is less or equal than `i`.
fn DOUBLE_EQUAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE_EQUAL, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NOT_EQUAL in current rule
fn NOT_EQUAL_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NOT_EQUAL, starting from 0.
/// Returns `None` if number of children corresponding to token NOT_EQUAL is less or equal than `i`.
fn NOT_EQUAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NOT_EQUAL, i)
}

}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equalityExpression(&mut self,)
	-> Result<Rc<EqualityExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_equalityExpression);
        let mut _localctx: Rc<EqualityExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relationalExpression*/
			recog.base.set_state(857);
			recog.relationalExpression()?;

			recog.base.set_state(862);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(89,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(858);
					_la = recog.base.input.la(1);
					if { !(_la==NOT_EQUAL || _la==DOUBLE_EQUAL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule relationalExpression*/
					recog.base.set_state(859);
					recog.relationalExpression()?;

					}
					} 
				}
				recog.base.set_state(864);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(89,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relationalExpression ----------------
pub type RelationalExpressionContextAll<'input> = RelationalExpressionContext<'input>;


pub type RelationalExpressionContext<'input> = BaseParserRuleContext<'input,RelationalExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct RelationalExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for RelationalExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for RelationalExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relationalExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_relationalExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for RelationalExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_relationalExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationalExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relationalExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relationalExpression }
}
antlr_rust::tid!{RelationalExpressionContextExt<'a>}

impl<'input> RelationalExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationalExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationalExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationalExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<RelationalExpressionContextExt<'input>>{

fn bitShiftExpression_all(&self) ->  Vec<Rc<BitShiftExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn bitShiftExpression(&self, i: usize) -> Option<Rc<BitShiftExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LESS_EQUAL in current rule
fn LESS_EQUAL_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LESS_EQUAL, starting from 0.
/// Returns `None` if number of children corresponding to token LESS_EQUAL is less or equal than `i`.
fn LESS_EQUAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LESS_EQUAL, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GREATER_EQUAL in current rule
fn GREATER_EQUAL_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GREATER_EQUAL, starting from 0.
/// Returns `None` if number of children corresponding to token GREATER_EQUAL is less or equal than `i`.
fn GREATER_EQUAL(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(GREATER_EQUAL, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LESS in current rule
fn LESS_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LESS, starting from 0.
/// Returns `None` if number of children corresponding to token LESS is less or equal than `i`.
fn LESS(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LESS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GREATER in current rule
fn GREATER_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GREATER, starting from 0.
/// Returns `None` if number of children corresponding to token GREATER is less or equal than `i`.
fn GREATER(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(GREATER, i)
}

}

impl<'input> RelationalExpressionContextAttrs<'input> for RelationalExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relationalExpression(&mut self,)
	-> Result<Rc<RelationalExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationalExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_relationalExpression);
        let mut _localctx: Rc<RelationalExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule bitShiftExpression*/
			recog.base.set_state(865);
			recog.bitShiftExpression()?;

			recog.base.set_state(870);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(866);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << GREATER) | (1usize << GREATER_EQUAL) | (1usize << LESS) | (1usize << LESS_EQUAL))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule bitShiftExpression*/
					recog.base.set_state(867);
					recog.bitShiftExpression()?;

					}
					} 
				}
				recog.base.set_state(872);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bitShiftExpression ----------------
pub type BitShiftExpressionContextAll<'input> = BitShiftExpressionContext<'input>;


pub type BitShiftExpressionContext<'input> = BaseParserRuleContext<'input,BitShiftExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct BitShiftExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for BitShiftExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for BitShiftExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bitShiftExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_bitShiftExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for BitShiftExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_bitShiftExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BitShiftExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bitShiftExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bitShiftExpression }
}
antlr_rust::tid!{BitShiftExpressionContextExt<'a>}

impl<'input> BitShiftExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BitShiftExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BitShiftExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BitShiftExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<BitShiftExpressionContextExt<'input>>{

fn additiveExpression_all(&self) ->  Vec<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn additiveExpression(&self, i: usize) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SHIFT_LEFT in current rule
fn SHIFT_LEFT_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SHIFT_LEFT, starting from 0.
/// Returns `None` if number of children corresponding to token SHIFT_LEFT is less or equal than `i`.
fn SHIFT_LEFT(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_LEFT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SHIFT_RIGHT in current rule
fn SHIFT_RIGHT_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SHIFT_RIGHT, starting from 0.
/// Returns `None` if number of children corresponding to token SHIFT_RIGHT is less or equal than `i`.
fn SHIFT_RIGHT(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SHIFT_RIGHT, i)
}

}

impl<'input> BitShiftExpressionContextAttrs<'input> for BitShiftExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bitShiftExpression(&mut self,)
	-> Result<Rc<BitShiftExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BitShiftExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_bitShiftExpression);
        let mut _localctx: Rc<BitShiftExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule additiveExpression*/
			recog.base.set_state(873);
			recog.additiveExpression()?;

			recog.base.set_state(878);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(91,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(874);
					_la = recog.base.input.la(1);
					if { !(_la==SHIFT_LEFT || _la==SHIFT_RIGHT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule additiveExpression*/
					recog.base.set_state(875);
					recog.additiveExpression()?;

					}
					} 
				}
				recog.base.set_state(880);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(91,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- additiveExpression ----------------
pub type AdditiveExpressionContextAll<'input> = AdditiveExpressionContext<'input>;


pub type AdditiveExpressionContext<'input> = BaseParserRuleContext<'input,AdditiveExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for AdditiveExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for AdditiveExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_additiveExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_additiveExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for AdditiveExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_additiveExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}
antlr_rust::tid!{AdditiveExpressionContextExt<'a>}

impl<'input> AdditiveExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditiveExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<AdditiveExpressionContextExt<'input>>{

fn multiplicativeExpression_all(&self) ->  Vec<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplicativeExpression(&self, i: usize) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PLUS in current rule
fn PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token PLUS is less or equal than `i`.
fn PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PLUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
/// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(MINUS, i)
}

}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn additiveExpression(&mut self,)
	-> Result<Rc<AdditiveExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditiveExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_additiveExpression);
        let mut _localctx: Rc<AdditiveExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplicativeExpression*/
			recog.base.set_state(881);
			recog.multiplicativeExpression()?;

			recog.base.set_state(886);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(882);
					_la = recog.base.input.la(1);
					if { !(_la==MINUS || _la==PLUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(883);
					recog.multiplicativeExpression()?;

					}
					} 
				}
				recog.base.set_state(888);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiplicativeExpression ----------------
pub type MultiplicativeExpressionContextAll<'input> = MultiplicativeExpressionContext<'input>;


pub type MultiplicativeExpressionContext<'input> = BaseParserRuleContext<'input,MultiplicativeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicativeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for MultiplicativeExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiplicativeExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_multiplicativeExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for MultiplicativeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}
antlr_rust::tid!{MultiplicativeExpressionContextExt<'a>}

impl<'input> MultiplicativeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplicativeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicativeExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicativeExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<MultiplicativeExpressionContextExt<'input>>{

fn powExpression_all(&self) ->  Vec<Rc<PowExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn powExpression(&self, i: usize) -> Option<Rc<PowExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token STAR in current rule
fn STAR_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STAR, starting from 0.
/// Returns `None` if number of children corresponding to token STAR is less or equal than `i`.
fn STAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(STAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SLASH in current rule
fn SLASH_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SLASH, starting from 0.
/// Returns `None` if number of children corresponding to token SLASH is less or equal than `i`.
fn SLASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(SLASH, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MOD in current rule
fn MOD_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MOD, starting from 0.
/// Returns `None` if number of children corresponding to token MOD is less or equal than `i`.
fn MOD(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(MOD, i)
}

}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplicativeExpression(&mut self,)
	-> Result<Rc<MultiplicativeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicativeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_multiplicativeExpression);
        let mut _localctx: Rc<MultiplicativeExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule powExpression*/
			recog.base.set_state(889);
			recog.powExpression()?;

			recog.base.set_state(894);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(890);
					_la = recog.base.input.la(1);
					if { !(((((_la - 26)) & !0x3f) == 0 && ((1usize << (_la - 26)) & ((1usize << (SLASH - 26)) | (1usize << (STAR - 26)) | (1usize << (MOD - 26)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule powExpression*/
					recog.base.set_state(891);
					recog.powExpression()?;

					}
					} 
				}
				recog.base.set_state(896);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- powExpression ----------------
pub type PowExpressionContextAll<'input> = PowExpressionContext<'input>;


pub type PowExpressionContext<'input> = BaseParserRuleContext<'input,PowExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PowExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PowExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PowExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_powExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_powExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PowExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_powExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powExpression }
}
antlr_rust::tid!{PowExpressionContextExt<'a>}

impl<'input> PowExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PowExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PowExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PowExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PowExpressionContextExt<'input>>{

fn unaryExpression_all(&self) ->  Vec<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unaryExpression(&self, i: usize) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token TIMES_TIMES in current rule
fn TIMES_TIMES_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TIMES_TIMES, starting from 0.
/// Returns `None` if number of children corresponding to token TIMES_TIMES is less or equal than `i`.
fn TIMES_TIMES(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(TIMES_TIMES, i)
}

}

impl<'input> PowExpressionContextAttrs<'input> for PowExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn powExpression(&mut self,)
	-> Result<Rc<PowExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PowExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_powExpression);
        let mut _localctx: Rc<PowExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryExpression*/
			recog.base.set_state(897);
			recog.unaryExpression()?;

			recog.base.set_state(902);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(94,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(898);
					recog.base.match_token(TIMES_TIMES,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(899);
					recog.unaryExpression()?;

					}
					} 
				}
				recog.base.set_state(904);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(94,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryExpression ----------------
pub type UnaryExpressionContextAll<'input> = UnaryExpressionContext<'input>;


pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for UnaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_unaryExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for UnaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_unaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}
antlr_rust::tid!{UnaryExpressionContextExt<'a>}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<UnaryExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
fn unaryExpressionNotPlusMinus(&self) -> Option<Rc<UnaryExpressionNotPlusMinusContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryExpression(&mut self,)
	-> Result<Rc<UnaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(910);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PLUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(905);
					recog.base.match_token(PLUS,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(906);
					recog.unaryExpression()?;

					}
				}

			 MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(907);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(908);
					recog.unaryExpression()?;

					}
				}

			 LPAREN | NOT | BIT_NEGATION | INITIAL | FRESH | ONE | RETURN | INTLITERAL |
			 BVLITERAL | REALLITERAL | CHARLITERAL | STRINGLITERAL | BOOLLITERAL |
			 ENUMLITERAL | IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule unaryExpressionNotPlusMinus*/
					recog.base.set_state(909);
					recog.unaryExpressionNotPlusMinus()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryExpressionNotPlusMinus ----------------
pub type UnaryExpressionNotPlusMinusContextAll<'input> = UnaryExpressionNotPlusMinusContext<'input>;


pub type UnaryExpressionNotPlusMinusContext<'input> = BaseParserRuleContext<'input,UnaryExpressionNotPlusMinusContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionNotPlusMinusContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for UnaryExpressionNotPlusMinusContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for UnaryExpressionNotPlusMinusContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryExpressionNotPlusMinus(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_unaryExpressionNotPlusMinus(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for UnaryExpressionNotPlusMinusContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_unaryExpressionNotPlusMinus(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionNotPlusMinusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpressionNotPlusMinus }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpressionNotPlusMinus }
}
antlr_rust::tid!{UnaryExpressionNotPlusMinusContextExt<'a>}

impl<'input> UnaryExpressionNotPlusMinusContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryExpressionNotPlusMinusContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionNotPlusMinusContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryExpressionNotPlusMinusContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<UnaryExpressionNotPlusMinusContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BIT_NEGATION
/// Returns `None` if there is no child corresponding to token BIT_NEGATION
fn BIT_NEGATION(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(BIT_NEGATION, 0)
}
fn primary(&self) -> Option<Rc<PrimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MINUS_MINUS
/// Returns `None` if there is no child corresponding to token MINUS_MINUS
fn MINUS_MINUS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(MINUS_MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS_PLUS
/// Returns `None` if there is no child corresponding to token PLUS_PLUS
fn PLUS_PLUS(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PLUS_PLUS, 0)
}

}

impl<'input> UnaryExpressionNotPlusMinusContextAttrs<'input> for UnaryExpressionNotPlusMinusContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryExpressionNotPlusMinus(&mut self,)
	-> Result<Rc<UnaryExpressionNotPlusMinusContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionNotPlusMinusContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_unaryExpressionNotPlusMinus);
        let mut _localctx: Rc<UnaryExpressionNotPlusMinusContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(920);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(912);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(913);
					recog.unaryExpression()?;

					}
				}

			 BIT_NEGATION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(914);
					recog.base.match_token(BIT_NEGATION,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(915);
					recog.unaryExpression()?;

					}
				}

			 LPAREN | INITIAL | FRESH | ONE | RETURN | INTLITERAL | BVLITERAL | REALLITERAL |
			 CHARLITERAL | STRINGLITERAL | BOOLLITERAL | ENUMLITERAL | IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule primary*/
					recog.base.set_state(916);
					recog.primary()?;

					recog.base.set_state(918);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(96,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(917);
							_la = recog.base.input.la(1);
							if { !(_la==PLUS_PLUS || _la==MINUS_MINUS) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- oneExpr ----------------
pub type OneExprContextAll<'input> = OneExprContext<'input>;


pub type OneExprContext<'input> = BaseParserRuleContext<'input,OneExprContextExt<'input>>;

#[derive(Clone)]
pub struct OneExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for OneExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for OneExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_oneExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_oneExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for OneExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_oneExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OneExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_oneExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_oneExpr }
}
antlr_rust::tid!{OneExprContextExt<'a>}

impl<'input> OneExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OneExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OneExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OneExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<OneExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ONE
/// Returns `None` if there is no child corresponding to token ONE
fn ONE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ONE, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> OneExprContextAttrs<'input> for OneExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn oneExpr(&mut self,)
	-> Result<Rc<OneExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OneExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_oneExpr);
        let mut _localctx: Rc<OneExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(922);
			recog.base.match_token(ONE,&mut recog.err_handler)?;

			recog.base.set_state(923);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(924);
			recog.expression()?;

			recog.base.set_state(927); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(925);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(926);
				recog.expression()?;

				}
				}
				recog.base.set_state(929); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==COMMA) {break}
			}
			recog.base.set_state(931);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- freshExpr ----------------
pub type FreshExprContextAll<'input> = FreshExprContext<'input>;


pub type FreshExprContext<'input> = BaseParserRuleContext<'input,FreshExprContextExt<'input>>;

#[derive(Clone)]
pub struct FreshExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FreshExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FreshExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_freshExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_freshExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FreshExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_freshExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for FreshExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_freshExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_freshExpr }
}
antlr_rust::tid!{FreshExprContextExt<'a>}

impl<'input> FreshExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FreshExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FreshExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FreshExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FreshExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FRESH
/// Returns `None` if there is no child corresponding to token FRESH
fn FRESH(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(FRESH, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> FreshExprContextAttrs<'input> for FreshExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn freshExpr(&mut self,)
	-> Result<Rc<FreshExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FreshExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_freshExpr);
        let mut _localctx: Rc<FreshExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(933);
			recog.base.match_token(FRESH,&mut recog.err_handler)?;

			recog.base.set_state(934);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(935);
			recog.identifier()?;

			recog.base.set_state(936);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- initialExpr ----------------
pub type InitialExprContextAll<'input> = InitialExprContext<'input>;


pub type InitialExprContext<'input> = BaseParserRuleContext<'input,InitialExprContextExt<'input>>;

#[derive(Clone)]
pub struct InitialExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for InitialExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for InitialExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_initialExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_initialExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for InitialExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_initialExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitialExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initialExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initialExpr }
}
antlr_rust::tid!{InitialExprContextExt<'a>}

impl<'input> InitialExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitialExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitialExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitialExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<InitialExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INITIAL
/// Returns `None` if there is no child corresponding to token INITIAL
fn INITIAL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(INITIAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn dotIdentifierExpr(&self) -> Option<Rc<DotIdentifierExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> InitialExprContextAttrs<'input> for InitialExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initialExpr(&mut self,)
	-> Result<Rc<InitialExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitialExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_initialExpr);
        let mut _localctx: Rc<InitialExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(938);
			recog.base.match_token(INITIAL,&mut recog.err_handler)?;

			recog.base.set_state(939);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule dotIdentifierExpr*/
			recog.base.set_state(940);
			recog.dotIdentifierExpr()?;

			recog.base.set_state(941);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prevExpr ----------------
pub type PrevExprContextAll<'input> = PrevExprContext<'input>;


pub type PrevExprContext<'input> = BaseParserRuleContext<'input,PrevExprContextExt<'input>>;

#[derive(Clone)]
pub struct PrevExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PrevExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PrevExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prevExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_prevExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PrevExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_prevExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrevExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prevExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prevExpr }
}
antlr_rust::tid!{PrevExprContextExt<'a>}

impl<'input> PrevExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrevExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrevExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrevExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PrevExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PREV
/// Returns `None` if there is no child corresponding to token PREV
fn PREV(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(PREV, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> PrevExprContextAttrs<'input> for PrevExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prevExpr(&mut self,)
	-> Result<Rc<PrevExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrevExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_prevExpr);
        let mut _localctx: Rc<PrevExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(943);
			recog.base.match_token(PREV,&mut recog.err_handler)?;

			recog.base.set_state(944);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(945);
			recog.identifier()?;

			recog.base.set_state(946);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionDeclaration ----------------
pub type FunctionDeclarationContextAll<'input> = FunctionDeclarationContext<'input>;


pub type FunctionDeclarationContext<'input> = BaseParserRuleContext<'input,FunctionDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FunctionDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FunctionDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionDeclaration(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_functionDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FunctionDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_functionDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDeclaration }
}
antlr_rust::tid!{FunctionDeclarationContextExt<'a>}

impl<'input> FunctionDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDeclarationContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FunctionDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNCTION
/// Returns `None` if there is no child corresponding to token FUNCTION
fn FUNCTION(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(FUNCTION, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn primitiveBvType(&self) -> Option<Rc<PrimitiveBvTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionBodyScope(&self) -> Option<Rc<FunctionBodyScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionDeclarationContextAttrs<'input> for FunctionDeclarationContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDeclaration(&mut self,)
	-> Result<Rc<FunctionDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_functionDeclaration);
        let mut _localctx: Rc<FunctionDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(948);
			recog.base.match_token(FUNCTION,&mut recog.err_handler)?;

			{
			/*InvokeRule identifier*/
			recog.base.set_state(949);
			recog.identifier()?;

			}
			recog.base.set_state(950);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule primitiveBvType*/
			recog.base.set_state(951);
			recog.primitiveBvType()?;

			/*InvokeRule functionBodyScope*/
			recog.base.set_state(952);
			recog.functionBodyScope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionBodyScope ----------------
pub type FunctionBodyScopeContextAll<'input> = FunctionBodyScopeContext<'input>;


pub type FunctionBodyScopeContext<'input> = BaseParserRuleContext<'input,FunctionBodyScopeContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionBodyScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FunctionBodyScopeContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FunctionBodyScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionBodyScope(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_functionBodyScope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FunctionBodyScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_functionBodyScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionBodyScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionBodyScope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionBodyScope }
}
antlr_rust::tid!{FunctionBodyScopeContextExt<'a>}

impl<'input> FunctionBodyScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionBodyScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionBodyScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionBodyScopeContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FunctionBodyScopeContextExt<'input>>{

fn functionParamsDecl(&self) -> Option<Rc<FunctionParamsDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn localVariableGroup_all(&self) ->  Vec<Rc<LocalVariableGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn localVariableGroup(&self, i: usize) -> Option<Rc<LocalVariableGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FunctionBodyScopeContextAttrs<'input> for FunctionBodyScopeContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionBodyScope(&mut self,)
	-> Result<Rc<FunctionBodyScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionBodyScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_functionBodyScope);
        let mut _localctx: Rc<FunctionBodyScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule functionParamsDecl*/
			recog.base.set_state(954);
			recog.functionParamsDecl()?;

			recog.base.set_state(955);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(959);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (INT - 73)) | (1usize << (BOOL - 73)) | (1usize << (REAL - 73)) | (1usize << (STRING - 73)) | (1usize << (ENUM - 73)))) != 0) || _la==BV {
				{
				{
				/*InvokeRule localVariableGroup*/
				recog.base.set_state(956);
				recog.localVariableGroup()?;

				}
				}
				recog.base.set_state(961);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(963); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(962);
				recog.statement()?;

				}
				}
				recog.base.set_state(965); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & ((1usize << (LPAREN - 19)) | (1usize << (MINUS - 19)) | (1usize << (PLUS - 19)) | (1usize << (NOT - 19)) | (1usize << (BIT_NEGATION - 19)))) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & ((1usize << (INITIAL - 98)) | (1usize << (FRESH - 98)) | (1usize << (ONE - 98)) | (1usize << (RETURN - 98)) | (1usize << (INTLITERAL - 98)) | (1usize << (BVLITERAL - 98)) | (1usize << (REALLITERAL - 98)) | (1usize << (CHARLITERAL - 98)) | (1usize << (STRINGLITERAL - 98)) | (1usize << (BOOLLITERAL - 98)) | (1usize << (ENUMLITERAL - 98)) | (1usize << (IDENT - 98)))) != 0)) {break}
			}
			recog.base.set_state(967);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionParamsDecl ----------------
pub type FunctionParamsDeclContextAll<'input> = FunctionParamsDeclContext<'input>;


pub type FunctionParamsDeclContext<'input> = BaseParserRuleContext<'input,FunctionParamsDeclContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionParamsDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FunctionParamsDeclContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FunctionParamsDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionParamsDecl(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_functionParamsDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FunctionParamsDeclContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_functionParamsDecl(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionParamsDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionParamsDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionParamsDecl }
}
antlr_rust::tid!{FunctionParamsDeclContextExt<'a>}

impl<'input> FunctionParamsDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionParamsDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionParamsDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionParamsDeclContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FunctionParamsDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn functionParam_all(&self) ->  Vec<Rc<FunctionParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionParam(&self, i: usize) -> Option<Rc<FunctionParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FunctionParamsDeclContextAttrs<'input> for FunctionParamsDeclContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionParamsDecl(&mut self,)
	-> Result<Rc<FunctionParamsDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionParamsDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_functionParamsDecl);
        let mut _localctx: Rc<FunctionParamsDeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(969);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(971);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENT {
				{
				/*InvokeRule functionParam*/
				recog.base.set_state(970);
				recog.functionParam()?;

				}
			}

			recog.base.set_state(977);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(973);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule functionParam*/
				recog.base.set_state(974);
				recog.functionParam()?;

				}
				}
				recog.base.set_state(979);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(980);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionParam ----------------
pub type FunctionParamContextAll<'input> = FunctionParamContext<'input>;


pub type FunctionParamContext<'input> = BaseParserRuleContext<'input,FunctionParamContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FunctionParamContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FunctionParamContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionParam(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_functionParam(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FunctionParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_functionParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionParam }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionParam }
}
antlr_rust::tid!{FunctionParamContextExt<'a>}

impl<'input> FunctionParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionParamContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FunctionParamContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn primitiveBvType(&self) -> Option<Rc<PrimitiveBvTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionParamContextAttrs<'input> for FunctionParamContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionParam(&mut self,)
	-> Result<Rc<FunctionParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_functionParam);
        let mut _localctx: Rc<FunctionParamContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(982);
			recog.identifier()?;

			recog.base.set_state(983);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule primitiveBvType*/
			recog.base.set_state(984);
			recog.primitiveBvType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnExpr ----------------
pub type ReturnExprContextAll<'input> = ReturnExprContext<'input>;


pub type ReturnExprContext<'input> = BaseParserRuleContext<'input,ReturnExprContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ReturnExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ReturnExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_returnExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ReturnExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_returnExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnExpr }
}
antlr_rust::tid!{ReturnExprContextExt<'a>}

impl<'input> ReturnExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ReturnExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnExprContextAttrs<'input> for ReturnExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnExpr(&mut self,)
	-> Result<Rc<ReturnExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_returnExpr);
        let mut _localctx: Rc<ReturnExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(986);
			recog.base.match_token(RETURN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(987);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primary ----------------
pub type PrimaryContextAll<'input> = PrimaryContext<'input>;


pub type PrimaryContext<'input> = BaseParserRuleContext<'input,PrimaryContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for PrimaryContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for PrimaryContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primary(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_primary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for PrimaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_primary(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}
antlr_rust::tid!{PrimaryContextExt<'a>}

impl<'input> PrimaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<PrimaryContextExt<'input>>{

fn parExpression(&self) -> Option<Rc<ParExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dotIdentifierExpr(&self) -> Option<Rc<DotIdentifierExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn initialExpr(&self) -> Option<Rc<InitialExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn freshExpr(&self) -> Option<Rc<FreshExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn oneExpr(&self) -> Option<Rc<OneExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnExpr(&self) -> Option<Rc<ReturnExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn funCall(&self) -> Option<Rc<FunCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryContextAttrs<'input> for PrimaryContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary(&mut self,)
	-> Result<Rc<PrimaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_primary);
        let mut _localctx: Rc<PrimaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(997);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(103,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule parExpression*/
					recog.base.set_state(989);
					recog.parExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule dotIdentifierExpr*/
					recog.base.set_state(990);
					recog.dotIdentifierExpr()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule literal*/
					recog.base.set_state(991);
					recog.literal()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule initialExpr*/
					recog.base.set_state(992);
					recog.initialExpr()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule freshExpr*/
					recog.base.set_state(993);
					recog.freshExpr()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule oneExpr*/
					recog.base.set_state(994);
					recog.oneExpr()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule returnExpr*/
					recog.base.set_state(995);
					recog.returnExpr()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule funCall*/
					recog.base.set_state(996);
					recog.funCall()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- dotIdentifierExpr ----------------
pub type DotIdentifierExprContextAll<'input> = DotIdentifierExprContext<'input>;


pub type DotIdentifierExprContext<'input> = BaseParserRuleContext<'input,DotIdentifierExprContextExt<'input>>;

#[derive(Clone)]
pub struct DotIdentifierExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for DotIdentifierExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for DotIdentifierExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dotIdentifierExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_dotIdentifierExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for DotIdentifierExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_dotIdentifierExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for DotIdentifierExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dotIdentifierExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dotIdentifierExpr }
}
antlr_rust::tid!{DotIdentifierExprContextExt<'a>}

impl<'input> DotIdentifierExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DotIdentifierExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DotIdentifierExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DotIdentifierExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<DotIdentifierExprContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> DotIdentifierExprContextAttrs<'input> for DotIdentifierExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dotIdentifierExpr(&mut self,)
	-> Result<Rc<DotIdentifierExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DotIdentifierExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_dotIdentifierExpr);
        let mut _localctx: Rc<DotIdentifierExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(999);
			recog.identifier()?;

			recog.base.set_state(1002);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(1000);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(1001);
				recog.identifier()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parExpression ----------------
pub type ParExpressionContextAll<'input> = ParExpressionContext<'input>;


pub type ParExpressionContext<'input> = BaseParserRuleContext<'input,ParExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ParExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for ParExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for ParExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parExpression(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_parExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for ParExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_parExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parExpression }
}
antlr_rust::tid!{ParExpressionContextExt<'a>}

impl<'input> ParExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParExpressionContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<ParExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ParExpressionContextAttrs<'input> for ParExpressionContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parExpression(&mut self,)
	-> Result<Rc<ParExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_parExpression);
        let mut _localctx: Rc<ParExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1004);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1005);
			recog.expression()?;

			recog.base.set_state(1006);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- funCall ----------------
pub type FunCallContextAll<'input> = FunCallContext<'input>;


pub type FunCallContext<'input> = BaseParserRuleContext<'input,FunCallContextExt<'input>>;

#[derive(Clone)]
pub struct FunCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for FunCallContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for FunCallContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_funCall(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_funCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for FunCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_funCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funCall }
}
antlr_rust::tid!{FunCallContextExt<'a>}

impl<'input> FunCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunCallContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<FunCallContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,CycloneParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FunCallContextAttrs<'input> for FunCallContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn funCall(&mut self,)
	-> Result<Rc<FunCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_funCall);
        let mut _localctx: Rc<FunCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(1008);
			recog.identifier()?;

			recog.base.set_state(1009);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1010);
			recog.expression()?;

			recog.base.set_state(1015);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1011);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1012);
				recog.expression()?;

				}
				}
				recog.base.set_state(1017);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1018);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iteStatement ----------------
pub type IteStatementContextAll<'input> = IteStatementContext<'input>;


pub type IteStatementContext<'input> = BaseParserRuleContext<'input,IteStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IteStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for IteStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for IteStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iteStatement(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_iteStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for IteStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_iteStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IteStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iteStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iteStatement }
}
antlr_rust::tid!{IteStatementContextExt<'a>}

impl<'input> IteStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IteStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IteStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IteStatementContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<IteStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn parExpression(&self) -> Option<Rc<ParExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}

}

impl<'input> IteStatementContextAttrs<'input> for IteStatementContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iteStatement(&mut self,)
	-> Result<Rc<IteStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IteStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_iteStatement);
        let mut _localctx: Rc<IteStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1020);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			/*InvokeRule parExpression*/
			recog.base.set_state(1021);
			recog.parExpression()?;

			/*InvokeRule statement*/
			recog.base.set_state(1022);
			recog.statement()?;

			recog.base.set_state(1025);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ELSE {
				{
				recog.base.set_state(1023);
				recog.base.match_token(ELSE,&mut recog.err_handler)?;

				/*InvokeRule statement*/
				recog.base.set_state(1024);
				recog.statement()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationExpr ----------------
pub type AnnotationExprContextAll<'input> = AnnotationExprContext<'input>;


pub type AnnotationExprContext<'input> = BaseParserRuleContext<'input,AnnotationExprContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CycloneParserContext<'input> for AnnotationExprContext<'input>{}

impl<'input,'a> Listenable<dyn CycloneParserListener<'input> + 'a> for AnnotationExprContext<'input>{
		fn enter(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationExpr(self);
		}
		fn exit(&self,listener: &mut (dyn CycloneParserListener<'input> + 'a)) {
			listener.exit_annotationExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn CycloneParserVisitor<'input> + 'a> for AnnotationExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn CycloneParserVisitor<'input> + 'a)) {
		visitor.visit_annotationExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnnotationExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CycloneParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationExpr }
}
antlr_rust::tid!{AnnotationExprContextExt<'a>}

impl<'input> AnnotationExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn CycloneParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationExprContextAttrs<'input>: CycloneParserContext<'input> + BorrowMut<AnnotationExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT_SIGN
/// Returns `None` if there is no child corresponding to token AT_SIGN
fn AT_SIGN(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(AT_SIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token LABEL
/// Returns `None` if there is no child corresponding to token LABEL
fn LABEL(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(LABEL, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,CycloneParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationExprContextAttrs<'input> for AnnotationExprContext<'input>{}

impl<'input, I, H> CycloneParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationExpr(&mut self,)
	-> Result<Rc<AnnotationExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_annotationExpr);
        let mut _localctx: Rc<AnnotationExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1027);
			recog.base.match_token(AT_SIGN,&mut recog.err_handler)?;

			recog.base.set_state(1028);
			recog.base.match_token(LABEL,&mut recog.err_handler)?;

			recog.base.set_state(1029);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(1030);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x7b\u{40b}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x03\x02\x03\
	\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\
	\x05\x07\x05\u{da}\x0a\x05\x0c\x05\x0e\x05\u{dd}\x0b\x05\x03\x06\x07\x06\
	\u{e0}\x0a\x06\x0c\x06\x0e\x06\u{e3}\x0b\x06\x03\x07\x03\x07\x05\x07\u{e7}\
	\x0a\x07\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{ed}\x0a\x08\x03\x09\x07\
	\x09\u{f0}\x0a\x09\x0c\x09\x0e\x09\u{f3}\x0b\x09\x03\x09\x03\x09\x03\x09\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x07\x0b\u{101}\x0a\x0b\x0c\x0b\x0e\x0b\u{104}\x0b\x0b\x03\x0b\x07\x0b\u{107}\
	\x0a\x0b\x0c\x0b\x0e\x0b\u{10a}\x0b\x0b\x03\x0b\x07\x0b\u{10d}\x0a\x0b\x0c\
	\x0b\x0e\x0b\u{110}\x0b\x0b\x03\x0b\x07\x0b\u{113}\x0a\x0b\x0c\x0b\x0e\x0b\
	\u{116}\x0b\x0b\x03\x0b\x05\x0b\u{119}\x0a\x0b\x03\x0b\x03\x0b\x03\x0c\x07\
	\x0c\u{11e}\x0a\x0c\x0c\x0c\x0e\x0c\u{121}\x0b\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0d\x03\x0d\x07\x0d\u{129}\x0a\x0d\x0c\x0d\x0e\x0d\u{12c}\x0b\
	\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x05\x0e\u{132}\x0a\x0e\x03\x0e\x03\
	\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{13d}\
	\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{142}\x0a\x0f\x03\x0f\x03\x0f\
	\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x07\x11\u{14b}\x0a\x11\x0c\x11\
	\x0e\x11\u{14e}\x0b\x11\x03\x11\x03\x11\x05\x11\u{152}\x0a\x11\x03\x11\x03\
	\x11\x05\x11\u{156}\x0a\x11\x05\x11\u{158}\x0a\x11\x03\x12\x03\x12\x03\x12\
	\x03\x12\x07\x12\u{15e}\x0a\x12\x0c\x12\x0e\x12\u{161}\x0b\x12\x03\x12\x03\
	\x12\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{169}\x0a\x13\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{170}\x0a\x14\x0c\x14\x0e\x14\u{173}\
	\x0b\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x16\x07\x16\u{180}\x0a\x16\x0c\x16\x0e\x16\u{183}\x0b\
	\x16\x03\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x05\x17\u{18b}\x0a\
	\x17\x03\x17\x05\x17\u{18e}\x0a\x17\x03\x17\x05\x17\u{191}\x0a\x17\x03\x18\
	\x03\x18\x03\x18\x03\x18\x07\x18\u{197}\x0a\x18\x0c\x18\x0e\x18\u{19a}\x0b\
	\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x07\x19\u{1a1}\x0a\x19\x0c\
	\x19\x0e\x19\u{1a4}\x0b\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x07\x1b\u{1b0}\x0a\x1b\x0c\x1b\x0e\
	\x1b\u{1b3}\x0b\x1b\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{1ba}\
	\x0a\x1c\x0c\x1c\x0e\x1c\u{1bd}\x0b\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\
	\x03\x1d\x05\x1d\u{1c4}\x0a\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x21\x03\x21\x03\x22\
	\x03\x22\x03\x22\x07\x22\u{1d6}\x0a\x22\x0c\x22\x0e\x22\u{1d9}\x0b\x22\x03\
	\x23\x03\x23\x03\x23\x07\x23\u{1de}\x0a\x23\x0c\x23\x0e\x23\u{1e1}\x0b\x23\
	\x03\x24\x03\x24\x03\x24\x07\x24\u{1e6}\x0a\x24\x0c\x24\x0e\x24\u{1e9}\x0b\
	\x24\x03\x25\x03\x25\x03\x25\x03\x25\x05\x25\u{1ef}\x0a\x25\x03\x26\x03\
	\x26\x03\x26\x05\x26\u{1f4}\x0a\x26\x03\x27\x03\x27\x03\x27\x03\x27\x03\
	\x28\x03\x28\x05\x28\u{1fc}\x0a\x28\x03\x28\x03\x28\x05\x28\u{200}\x0a\x28\
	\x05\x28\u{202}\x0a\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\
	\x05\x28\u{20a}\x0a\x28\x03\x28\x03\x28\x05\x28\u{20e}\x0a\x28\x03\x28\x03\
	\x28\x05\x28\u{212}\x0a\x28\x03\x28\x03\x28\x05\x28\u{216}\x0a\x28\x05\x28\
	\u{218}\x0a\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\
	\x05\x28\u{221}\x0a\x28\x03\x28\x03\x28\x05\x28\u{225}\x0a\x28\x03\x28\x03\
	\x28\x05\x28\u{229}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x07\x29\u{231}\x0a\x29\x0c\x29\x0e\x29\u{234}\x0b\x29\x03\x29\x03\x29\
	\x05\x29\u{238}\x0a\x29\x05\x29\u{23a}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x07\x29\u{243}\x0a\x29\x0c\x29\x0e\x29\u{246}\
	\x0b\x29\x03\x29\x03\x29\x05\x29\u{24a}\x0a\x29\x05\x29\u{24c}\x0a\x29\x06\
	\x29\u{24e}\x0a\x29\x0d\x29\x0e\x29\u{24f}\x03\x29\x03\x29\x05\x29\u{254}\
	\x0a\x29\x03\x29\x03\x29\x05\x29\u{258}\x0a\x29\x05\x29\u{25a}\x0a\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x07\x29\u{263}\x0a\
	\x29\x0c\x29\x0e\x29\u{266}\x0b\x29\x03\x29\x03\x29\x05\x29\u{26a}\x0a\x29\
	\x05\x29\u{26c}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\
	\x03\x29\x07\x29\u{275}\x0a\x29\x0c\x29\x0e\x29\u{278}\x0b\x29\x03\x29\x03\
	\x29\x05\x29\u{27c}\x0a\x29\x05\x29\u{27e}\x0a\x29\x06\x29\u{280}\x0a\x29\
	\x0d\x29\x0e\x29\u{281}\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\
	\x03\x29\x05\x29\u{28b}\x0a\x29\x03\x29\x03\x29\x05\x29\u{28f}\x0a\x29\x05\
	\x29\u{291}\x0a\x29\x03\x2a\x03\x2a\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x03\
	\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x05\x2d\u{2a0}\x0a\
	\x2d\x03\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x31\x03\x31\x03\
	\x32\x03\x32\x03\x33\x03\x33\x03\x34\x03\x34\x03\x35\x03\x35\x03\x35\x03\
	\x35\x03\x35\x03\x36\x03\x36\x03\x36\x03\x36\x03\x37\x06\x37\u{2ba}\x0a\
	\x37\x0d\x37\x0e\x37\u{2bb}\x03\x38\x03\x38\x03\x38\x03\x38\x03\x39\x03\
	\x39\x03\x39\x03\x39\x03\x39\x07\x39\u{2c7}\x0a\x39\x0c\x39\x0e\x39\u{2ca}\
	\x0b\x39\x03\x39\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3b\x07\x3b\u{2d6}\x0a\x3b\x0c\x3b\x0e\x3b\u{2d9}\x0b\x3b\x03\
	\x3b\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x07\x3c\u{2e1}\x0a\x3c\x0c\
	\x3c\x0e\x3c\u{2e4}\x0b\x3c\x03\x3c\x03\x3c\x03\x3d\x03\x3d\x03\x3e\x03\
	\x3e\x03\x3e\x05\x3e\u{2ed}\x0a\x3e\x03\x3f\x03\x3f\x05\x3f\u{2f1}\x0a\x3f\
	\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x41\x03\x41\x03\x42\x03\x42\
	\x03\x42\x03\x42\x03\x42\x07\x42\u{2ff}\x0a\x42\x0c\x42\x0e\x42\u{302}\x0b\
	\x42\x03\x42\x03\x42\x03\x43\x03\x43\x03\x44\x03\x44\x03\x44\x05\x44\u{30b}\
	\x0a\x44\x03\x44\x05\x44\u{30e}\x0a\x44\x03\x45\x03\x45\x03\x45\x03\x46\
	\x03\x46\x03\x47\x05\x47\u{316}\x0a\x47\x03\x47\x03\x47\x03\x47\x05\x47\
	\u{31b}\x0a\x47\x03\x47\x03\x47\x03\x48\x05\x48\u{320}\x0a\x48\x03\x48\x03\
	\x48\x03\x49\x03\x49\x03\x49\x03\x4a\x03\x4a\x03\x4a\x05\x4a\u{32a}\x0a\
	\x4a\x03\x4b\x03\x4b\x03\x4b\x07\x4b\u{32f}\x0a\x4b\x0c\x4b\x0e\x4b\u{332}\
	\x0b\x4b\x03\x4c\x03\x4c\x03\x4c\x07\x4c\u{337}\x0a\x4c\x0c\x4c\x0e\x4c\
	\u{33a}\x0b\x4c\x03\x4d\x03\x4d\x03\x4d\x07\x4d\u{33f}\x0a\x4d\x0c\x4d\x0e\
	\x4d\u{342}\x0b\x4d\x03\x4e\x03\x4e\x03\x4e\x07\x4e\u{347}\x0a\x4e\x0c\x4e\
	\x0e\x4e\u{34a}\x0b\x4e\x03\x4f\x03\x4f\x03\x4f\x07\x4f\u{34f}\x0a\x4f\x0c\
	\x4f\x0e\x4f\u{352}\x0b\x4f\x03\x50\x03\x50\x03\x50\x07\x50\u{357}\x0a\x50\
	\x0c\x50\x0e\x50\u{35a}\x0b\x50\x03\x51\x03\x51\x03\x51\x07\x51\u{35f}\x0a\
	\x51\x0c\x51\x0e\x51\u{362}\x0b\x51\x03\x52\x03\x52\x03\x52\x07\x52\u{367}\
	\x0a\x52\x0c\x52\x0e\x52\u{36a}\x0b\x52\x03\x53\x03\x53\x03\x53\x07\x53\
	\u{36f}\x0a\x53\x0c\x53\x0e\x53\u{372}\x0b\x53\x03\x54\x03\x54\x03\x54\x07\
	\x54\u{377}\x0a\x54\x0c\x54\x0e\x54\u{37a}\x0b\x54\x03\x55\x03\x55\x03\x55\
	\x07\x55\u{37f}\x0a\x55\x0c\x55\x0e\x55\u{382}\x0b\x55\x03\x56\x03\x56\x03\
	\x56\x07\x56\u{387}\x0a\x56\x0c\x56\x0e\x56\u{38a}\x0b\x56\x03\x57\x03\x57\
	\x03\x57\x03\x57\x03\x57\x05\x57\u{391}\x0a\x57\x03\x58\x03\x58\x03\x58\
	\x03\x58\x03\x58\x03\x58\x05\x58\u{399}\x0a\x58\x05\x58\u{39b}\x0a\x58\x03\
	\x59\x03\x59\x03\x59\x03\x59\x03\x59\x06\x59\u{3a2}\x0a\x59\x0d\x59\x0e\
	\x59\u{3a3}\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\
	\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x03\
	\x5c\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5e\x03\x5e\x03\
	\x5e\x07\x5e\u{3c0}\x0a\x5e\x0c\x5e\x0e\x5e\u{3c3}\x0b\x5e\x03\x5e\x06\x5e\
	\u{3c6}\x0a\x5e\x0d\x5e\x0e\x5e\u{3c7}\x03\x5e\x03\x5e\x03\x5f\x03\x5f\x05\
	\x5f\u{3ce}\x0a\x5f\x03\x5f\x03\x5f\x07\x5f\u{3d2}\x0a\x5f\x0c\x5f\x0e\x5f\
	\u{3d5}\x0b\x5f\x03\x5f\x03\x5f\x03\x60\x03\x60\x03\x60\x03\x60\x03\x61\
	\x03\x61\x03\x61\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\
	\x03\x62\x05\x62\u{3e8}\x0a\x62\x03\x63\x03\x63\x03\x63\x05\x63\u{3ed}\x0a\
	\x63\x03\x64\x03\x64\x03\x64\x03\x64\x03\x65\x03\x65\x03\x65\x03\x65\x03\
	\x65\x07\x65\u{3f8}\x0a\x65\x0c\x65\x0e\x65\u{3fb}\x0b\x65\x03\x65\x03\x65\
	\x03\x66\x03\x66\x03\x66\x03\x66\x03\x66\x05\x66\u{404}\x0a\x66\x03\x67\
	\x03\x67\x03\x67\x03\x67\x03\x67\x03\x67\x02\x02\x68\x02\x04\x06\x08\x0a\
	\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\
	\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\
	\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\
	\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\
	\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\
	\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\
	\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\x02\x17\x03\x02\x37\x3e\x03\x02\x41\x42\x03\
	\x02\x3f\x40\x03\x02\x43\x45\x03\x02\x48\x49\x03\x02\x03\x04\x04\x02\x58\
	\x58\x60\x60\x05\x02\x59\x59\x62\x62\x6b\x6b\x04\x02\x5a\x5a\x5e\x5e\x03\
	\x02\x5c\x5d\x03\x02\x52\x55\x03\x02\x35\x36\x04\x02\x4b\x4d\x4f\x4f\x03\
	\x02\x68\x69\x04\x02\x0d\x0d\x2c\x31\x04\x02\x17\x17\x34\x34\x04\x02\x0e\
	\x0f\x13\x14\x03\x02\x32\x33\x04\x02\x16\x16\x1e\x1e\x04\x02\x1c\x1d\x29\
	\x29\x03\x02\x26\x27\x02\u{428}\x02\u{ce}\x03\x02\x02\x02\x04\u{d0}\x03\
	\x02\x02\x02\x06\u{d6}\x03\x02\x02\x02\x08\u{db}\x03\x02\x02\x02\x0a\u{e1}\
	\x03\x02\x02\x02\x0c\u{e6}\x03\x02\x02\x02\x0e\u{ec}\x03\x02\x02\x02\x10\
	\u{f1}\x03\x02\x02\x02\x12\u{f7}\x03\x02\x02\x02\x14\u{fb}\x03\x02\x02\x02\
	\x16\u{11f}\x03\x02\x02\x02\x18\u{126}\x03\x02\x02\x02\x1a\u{12f}\x03\x02\
	\x02\x02\x1c\u{135}\x03\x02\x02\x02\x1e\u{145}\x03\x02\x02\x02\x20\u{157}\
	\x03\x02\x02\x02\x22\u{159}\x03\x02\x02\x02\x24\u{164}\x03\x02\x02\x02\x26\
	\u{16a}\x03\x02\x02\x02\x28\u{176}\x03\x02\x02\x02\x2a\u{17a}\x03\x02\x02\
	\x02\x2c\u{187}\x03\x02\x02\x02\x2e\u{192}\x03\x02\x02\x02\x30\u{19b}\x03\
	\x02\x02\x02\x32\u{1a7}\x03\x02\x02\x02\x34\u{1ac}\x03\x02\x02\x02\x36\u{1b4}\
	\x03\x02\x02\x02\x38\u{1c0}\x03\x02\x02\x02\x3a\u{1c7}\x03\x02\x02\x02\x3c\
	\u{1cb}\x03\x02\x02\x02\x3e\u{1ce}\x03\x02\x02\x02\x40\u{1d0}\x03\x02\x02\
	\x02\x42\u{1d2}\x03\x02\x02\x02\x44\u{1da}\x03\x02\x02\x02\x46\u{1e2}\x03\
	\x02\x02\x02\x48\u{1ee}\x03\x02\x02\x02\x4a\u{1f3}\x03\x02\x02\x02\x4c\u{1f5}\
	\x03\x02\x02\x02\x4e\u{228}\x03\x02\x02\x02\x50\u{290}\x03\x02\x02\x02\x52\
	\u{292}\x03\x02\x02\x02\x54\u{294}\x03\x02\x02\x02\x56\u{296}\x03\x02\x02\
	\x02\x58\u{29f}\x03\x02\x02\x02\x5a\u{2a1}\x03\x02\x02\x02\x5c\u{2a3}\x03\
	\x02\x02\x02\x5e\u{2a5}\x03\x02\x02\x02\x60\u{2a7}\x03\x02\x02\x02\x62\u{2a9}\
	\x03\x02\x02\x02\x64\u{2ab}\x03\x02\x02\x02\x66\u{2ad}\x03\x02\x02\x02\x68\
	\u{2af}\x03\x02\x02\x02\x6a\u{2b4}\x03\x02\x02\x02\x6c\u{2b9}\x03\x02\x02\
	\x02\x6e\u{2bd}\x03\x02\x02\x02\x70\u{2c1}\x03\x02\x02\x02\x72\u{2cd}\x03\
	\x02\x02\x02\x74\u{2d1}\x03\x02\x02\x02\x76\u{2dc}\x03\x02\x02\x02\x78\u{2e7}\
	\x03\x02\x02\x02\x7a\u{2ec}\x03\x02\x02\x02\x7c\u{2f0}\x03\x02\x02\x02\x7e\
	\u{2f2}\x03\x02\x02\x02\u{80}\u{2f7}\x03\x02\x02\x02\u{82}\u{2f9}\x03\x02\
	\x02\x02\u{84}\u{305}\x03\x02\x02\x02\u{86}\u{307}\x03\x02\x02\x02\u{88}\
	\u{30f}\x03\x02\x02\x02\u{8a}\u{312}\x03\x02\x02\x02\u{8c}\u{315}\x03\x02\
	\x02\x02\u{8e}\u{31f}\x03\x02\x02\x02\u{90}\u{323}\x03\x02\x02\x02\u{92}\
	\u{326}\x03\x02\x02\x02\u{94}\u{32b}\x03\x02\x02\x02\u{96}\u{333}\x03\x02\
	\x02\x02\u{98}\u{33b}\x03\x02\x02\x02\u{9a}\u{343}\x03\x02\x02\x02\u{9c}\
	\u{34b}\x03\x02\x02\x02\u{9e}\u{353}\x03\x02\x02\x02\u{a0}\u{35b}\x03\x02\
	\x02\x02\u{a2}\u{363}\x03\x02\x02\x02\u{a4}\u{36b}\x03\x02\x02\x02\u{a6}\
	\u{373}\x03\x02\x02\x02\u{a8}\u{37b}\x03\x02\x02\x02\u{aa}\u{383}\x03\x02\
	\x02\x02\u{ac}\u{390}\x03\x02\x02\x02\u{ae}\u{39a}\x03\x02\x02\x02\u{b0}\
	\u{39c}\x03\x02\x02\x02\u{b2}\u{3a7}\x03\x02\x02\x02\u{b4}\u{3ac}\x03\x02\
	\x02\x02\u{b6}\u{3b1}\x03\x02\x02\x02\u{b8}\u{3b6}\x03\x02\x02\x02\u{ba}\
	\u{3bc}\x03\x02\x02\x02\u{bc}\u{3cb}\x03\x02\x02\x02\u{be}\u{3d8}\x03\x02\
	\x02\x02\u{c0}\u{3dc}\x03\x02\x02\x02\u{c2}\u{3e7}\x03\x02\x02\x02\u{c4}\
	\u{3e9}\x03\x02\x02\x02\u{c6}\u{3ee}\x03\x02\x02\x02\u{c8}\u{3f2}\x03\x02\
	\x02\x02\u{ca}\u{3fe}\x03\x02\x02\x02\u{cc}\u{405}\x03\x02\x02\x02\u{ce}\
	\u{cf}\x07\x78\x02\x02\u{cf}\x03\x03\x02\x02\x02\u{d0}\u{d1}\x07\x67\x02\
	\x02\u{d1}\u{d2}\x05\x06\x04\x02\u{d2}\u{d3}\x07\x0d\x02\x02\u{d3}\u{d4}\
	\x05\x58\x2d\x02\u{d4}\u{d5}\x07\x1b\x02\x02\u{d5}\x05\x03\x02\x02\x02\u{d6}\
	\u{d7}\x09\x02\x02\x02\u{d7}\x07\x03\x02\x02\x02\u{d8}\u{da}\x05\u{90}\x49\
	\x02\u{d9}\u{d8}\x03\x02\x02\x02\u{da}\u{dd}\x03\x02\x02\x02\u{db}\u{d9}\
	\x03\x02\x02\x02\u{db}\u{dc}\x03\x02\x02\x02\u{dc}\x09\x03\x02\x02\x02\u{dd}\
	\u{db}\x03\x02\x02\x02\u{de}\u{e0}\x05\x1a\x0e\x02\u{df}\u{de}\x03\x02\x02\
	\x02\u{e0}\u{e3}\x03\x02\x02\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e1}\u{e2}\
	\x03\x02\x02\x02\u{e2}\x0b\x03\x02\x02\x02\u{e3}\u{e1}\x03\x02\x02\x02\u{e4}\
	\u{e7}\x05\x38\x1d\x02\u{e5}\u{e7}\x05\x3a\x1e\x02\u{e6}\u{e4}\x03\x02\x02\
	\x02\u{e6}\u{e5}\x03\x02\x02\x02\u{e7}\x0d\x03\x02\x02\x02\u{e8}\u{ed}\x05\
	\x74\x3b\x02\u{e9}\u{ed}\x05\x70\x39\x02\u{ea}\u{ed}\x05\x68\x35\x02\u{eb}\
	\u{ed}\x05\u{b8}\x5d\x02\u{ec}\u{e8}\x03\x02\x02\x02\u{ec}\u{e9}\x03\x02\
	\x02\x02\u{ec}\u{ea}\x03\x02\x02\x02\u{ec}\u{eb}\x03\x02\x02\x02\u{ed}\x0f\
	\x03\x02\x02\x02\u{ee}\u{f0}\x05\x04\x03\x02\u{ef}\u{ee}\x03\x02\x02\x02\
	\u{f0}\u{f3}\x03\x02\x02\x02\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\u{f2}\x03\
	\x02\x02\x02\u{f2}\u{f4}\x03\x02\x02\x02\u{f3}\u{f1}\x03\x02\x02\x02\u{f4}\
	\u{f5}\x05\x12\x0a\x02\u{f5}\u{f6}\x07\x02\x02\x03\u{f6}\x11\x03\x02\x02\
	\x02\u{f7}\u{f8}\x09\x03\x02\x02\u{f8}\u{f9}\x05\x02\x02\x02\u{f9}\u{fa}\
	\x05\x14\x0b\x02\u{fa}\x13\x03\x02\x02\x02\u{fb}\u{102}\x07\x11\x02\x02\
	\u{fc}\u{101}\x05\x74\x3b\x02\u{fd}\u{101}\x05\x70\x39\x02\u{fe}\u{101}\
	\x05\x68\x35\x02\u{ff}\u{101}\x05\u{b8}\x5d\x02\u{100}\u{fc}\x03\x02\x02\
	\x02\u{100}\u{fd}\x03\x02\x02\x02\u{100}\u{fe}\x03\x02\x02\x02\u{100}\u{ff}\
	\x03\x02\x02\x02\u{101}\u{104}\x03\x02\x02\x02\u{102}\u{100}\x03\x02\x02\
	\x02\u{102}\u{103}\x03\x02\x02\x02\u{103}\u{108}\x03\x02\x02\x02\u{104}\
	\u{102}\x03\x02\x02\x02\u{105}\u{107}\x05\x16\x0c\x02\u{106}\u{105}\x03\
	\x02\x02\x02\u{107}\u{10a}\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\x02\
	\u{108}\u{109}\x03\x02\x02\x02\u{109}\u{10e}\x03\x02\x02\x02\u{10a}\u{108}\
	\x03\x02\x02\x02\u{10b}\u{10d}\x05\x1a\x0e\x02\u{10c}\u{10b}\x03\x02\x02\
	\x02\u{10d}\u{110}\x03\x02\x02\x02\u{10e}\u{10c}\x03\x02\x02\x02\u{10e}\
	\u{10f}\x03\x02\x02\x02\u{10f}\u{114}\x03\x02\x02\x02\u{110}\u{10e}\x03\
	\x02\x02\x02\u{111}\u{113}\x05\x24\x13\x02\u{112}\u{111}\x03\x02\x02\x02\
	\u{113}\u{116}\x03\x02\x02\x02\u{114}\u{112}\x03\x02\x02\x02\u{114}\u{115}\
	\x03\x02\x02\x02\u{115}\u{118}\x03\x02\x02\x02\u{116}\u{114}\x03\x02\x02\
	\x02\u{117}\u{119}\x05\x2a\x16\x02\u{118}\u{117}\x03\x02\x02\x02\u{118}\
	\u{119}\x03\x02\x02\x02\u{119}\u{11a}\x03\x02\x02\x02\u{11a}\u{11b}\x07\
	\x18\x02\x02\u{11b}\x15\x03\x02\x02\x02\u{11c}\u{11e}\x05\x56\x2c\x02\u{11d}\
	\u{11c}\x03\x02\x02\x02\u{11e}\u{121}\x03\x02\x02\x02\u{11f}\u{11d}\x03\
	\x02\x02\x02\u{11f}\u{120}\x03\x02\x02\x02\u{120}\u{122}\x03\x02\x02\x02\
	\u{121}\u{11f}\x03\x02\x02\x02\u{122}\u{123}\x09\x04\x02\x02\u{123}\u{124}\
	\x05\x02\x02\x02\u{124}\u{125}\x05\x18\x0d\x02\u{125}\x17\x03\x02\x02\x02\
	\u{126}\u{12a}\x07\x11\x02\x02\u{127}\u{129}\x05\u{90}\x49\x02\u{128}\u{127}\
	\x03\x02\x02\x02\u{129}\u{12c}\x03\x02\x02\x02\u{12a}\u{128}\x03\x02\x02\
	\x02\u{12a}\u{12b}\x03\x02\x02\x02\u{12b}\u{12d}\x03\x02\x02\x02\u{12c}\
	\u{12a}\x03\x02\x02\x02\u{12d}\u{12e}\x07\x18\x02\x02\u{12e}\x19\x03\x02\
	\x02\x02\u{12f}\u{131}\x09\x05\x02\x02\u{130}\u{132}\x05\x02\x02\x02\u{131}\
	\u{130}\x03\x02\x02\x02\u{131}\u{132}\x03\x02\x02\x02\u{132}\u{133}\x03\
	\x02\x02\x02\u{133}\u{134}\x05\x1c\x0f\x02\u{134}\x1b\x03\x02\x02\x02\u{135}\
	\u{136}\x07\x11\x02\x02\u{136}\u{137}\x05\x02\x02\x02\u{137}\u{138}\x05\
	\x1e\x10\x02\u{138}\u{139}\x05\x20\x11\x02\u{139}\u{13c}\x03\x02\x02\x02\
	\u{13a}\u{13b}\x09\x06\x02\x02\u{13b}\u{13d}\x05\x54\x2b\x02\u{13c}\u{13a}\
	\x03\x02\x02\x02\u{13c}\u{13d}\x03\x02\x02\x02\u{13d}\u{141}\x03\x02\x02\
	\x02\u{13e}\u{13f}\x05\u{88}\x45\x02\u{13f}\u{140}\x07\x1b\x02\x02\u{140}\
	\u{142}\x03\x02\x02\x02\u{141}\u{13e}\x03\x02\x02\x02\u{141}\u{142}\x03\
	\x02\x02\x02\u{142}\u{143}\x03\x02\x02\x02\u{143}\u{144}\x07\x18\x02\x02\
	\u{144}\x1d\x03\x02\x02\x02\u{145}\u{146}\x09\x07\x02\x02\u{146}\x1f\x03\
	\x02\x02\x02\u{147}\u{14c}\x05\x02\x02\x02\u{148}\u{149}\x07\x0a\x02\x02\
	\u{149}\u{14b}\x05\x02\x02\x02\u{14a}\u{148}\x03\x02\x02\x02\u{14b}\u{14e}\
	\x03\x02\x02\x02\u{14c}\u{14a}\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\
	\x02\u{14d}\u{158}\x03\x02\x02\x02\u{14e}\u{14c}\x03\x02\x02\x02\u{14f}\
	\u{151}\x07\x1d\x02\x02\u{150}\u{152}\x05\x22\x12\x02\u{151}\u{150}\x03\
	\x02\x02\x02\u{151}\u{152}\x03\x02\x02\x02\u{152}\u{158}\x03\x02\x02\x02\
	\u{153}\u{155}\x07\x1e\x02\x02\u{154}\u{156}\x05\x22\x12\x02\u{155}\u{154}\
	\x03\x02\x02\x02\u{155}\u{156}\x03\x02\x02\x02\u{156}\u{158}\x03\x02\x02\
	\x02\u{157}\u{147}\x03\x02\x02\x02\u{157}\u{14f}\x03\x02\x02\x02\u{157}\
	\u{153}\x03\x02\x02\x02\u{158}\x21\x03\x02\x02\x02\u{159}\u{15a}\x07\x12\
	\x02\x02\u{15a}\u{15f}\x05\x02\x02\x02\u{15b}\u{15c}\x07\x0a\x02\x02\u{15c}\
	\u{15e}\x05\x02\x02\x02\u{15d}\u{15b}\x03\x02\x02\x02\u{15e}\u{161}\x03\
	\x02\x02\x02\u{15f}\u{15d}\x03\x02\x02\x02\u{15f}\u{160}\x03\x02\x02\x02\
	\u{160}\u{162}\x03\x02\x02\x02\u{161}\u{15f}\x03\x02\x02\x02\u{162}\u{163}\
	\x07\x19\x02\x02\u{163}\x23\x03\x02\x02\x02\u{164}\u{165}\x07\x4a\x02\x02\
	\u{165}\u{166}\x05\x02\x02\x02\u{166}\u{168}\x05\x28\x15\x02\u{167}\u{169}\
	\x05\x26\x14\x02\u{168}\u{167}\x03\x02\x02\x02\u{168}\u{169}\x03\x02\x02\
	\x02\u{169}\x25\x03\x02\x02\x02\u{16a}\u{16b}\x07\x65\x02\x02\u{16b}\u{16c}\
	\x07\x15\x02\x02\u{16c}\u{171}\x05\x02\x02\x02\u{16d}\u{16e}\x07\x0a\x02\
	\x02\u{16e}\u{170}\x05\x02\x02\x02\u{16f}\u{16d}\x03\x02\x02\x02\u{170}\
	\u{173}\x03\x02\x02\x02\u{171}\u{16f}\x03\x02\x02\x02\u{171}\u{172}\x03\
	\x02\x02\x02\u{172}\u{174}\x03\x02\x02\x02\u{173}\u{171}\x03\x02\x02\x02\
	\u{174}\u{175}\x07\x1a\x02\x02\u{175}\x27\x03\x02\x02\x02\u{176}\u{177}\
	\x07\x11\x02\x02\u{177}\u{178}\x05\u{90}\x49\x02\u{178}\u{179}\x07\x18\x02\
	\x02\u{179}\x29\x03\x02\x02\x02\u{17a}\u{17b}\x07\x57\x02\x02\u{17b}\u{181}\
	\x07\x11\x02\x02\u{17c}\u{180}\x05\x38\x1d\x02\u{17d}\u{180}\x05\x3a\x1e\
	\x02\u{17e}\u{180}\x05\u{8c}\x47\x02\u{17f}\u{17c}\x03\x02\x02\x02\u{17f}\
	\u{17d}\x03\x02\x02\x02\u{17f}\u{17e}\x03\x02\x02\x02\u{180}\u{183}\x03\
	\x02\x02\x02\u{181}\u{17f}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\
	\u{182}\u{184}\x03\x02\x02\x02\u{183}\u{181}\x03\x02\x02\x02\u{184}\u{185}\
	\x05\x2c\x17\x02\u{185}\u{186}\x07\x18\x02\x02\u{186}\x2b\x03\x02\x02\x02\
	\u{187}\u{188}\x09\x08\x02\x02\u{188}\u{18a}\x05\x2e\x18\x02\u{189}\u{18b}\
	\x05\x32\x1a\x02\u{18a}\u{189}\x03\x02\x02\x02\u{18a}\u{18b}\x03\x02\x02\
	\x02\u{18b}\u{18d}\x03\x02\x02\x02\u{18c}\u{18e}\x05\x36\x1c\x02\u{18d}\
	\u{18c}\x03\x02\x02\x02\u{18d}\u{18e}\x03\x02\x02\x02\u{18e}\u{190}\x03\
	\x02\x02\x02\u{18f}\u{191}\x05\x30\x19\x02\u{190}\u{18f}\x03\x02\x02\x02\
	\u{190}\u{191}\x03\x02\x02\x02\u{191}\x2d\x03\x02\x02\x02\u{192}\u{193}\
	\x09\x09\x02\x02\u{193}\u{198}\x05\x5a\x2e\x02\u{194}\u{195}\x07\x0a\x02\
	\x02\u{195}\u{197}\x05\x5a\x2e\x02\u{196}\u{194}\x03\x02\x02\x02\u{197}\
	\u{19a}\x03\x02\x02\x02\u{198}\u{196}\x03\x02\x02\x02\u{198}\u{199}\x03\
	\x02\x02\x02\u{199}\x2f\x03\x02\x02\x02\u{19a}\u{198}\x03\x02\x02\x02\u{19b}\
	\u{19c}\x09\x0a\x02\x02\u{19c}\u{19d}\x07\x15\x02\x02\u{19d}\u{1a2}\x05\
	\x02\x02\x02\u{19e}\u{19f}\x07\x0a\x02\x02\u{19f}\u{1a1}\x05\x02\x02\x02\
	\u{1a0}\u{19e}\x03\x02\x02\x02\u{1a1}\u{1a4}\x03\x02\x02\x02\u{1a2}\u{1a0}\
	\x03\x02\x02\x02\u{1a2}\u{1a3}\x03\x02\x02\x02\u{1a3}\u{1a5}\x03\x02\x02\
	\x02\u{1a4}\u{1a2}\x03\x02\x02\x02\u{1a5}\u{1a6}\x07\x1a\x02\x02\u{1a6}\
	\x31\x03\x02\x02\x02\u{1a7}\u{1a8}\x09\x0b\x02\x02\u{1a8}\u{1a9}\x07\x15\
	\x02\x02\u{1a9}\u{1aa}\x05\x34\x1b\x02\u{1aa}\u{1ab}\x07\x1a\x02\x02\u{1ab}\
	\x33\x03\x02\x02\x02\u{1ac}\u{1b1}\x05\x3e\x20\x02\u{1ad}\u{1ae}\x07\x0a\
	\x02\x02\u{1ae}\u{1b0}\x05\x3e\x20\x02\u{1af}\u{1ad}\x03\x02\x02\x02\u{1b0}\
	\u{1b3}\x03\x02\x02\x02\u{1b1}\u{1af}\x03\x02\x02\x02\u{1b1}\u{1b2}\x03\
	\x02\x02\x02\u{1b2}\x35\x03\x02\x02\x02\u{1b3}\u{1b1}\x03\x02\x02\x02\u{1b4}\
	\u{1b5}\x07\x5f\x02\x02\u{1b5}\u{1b6}\x07\x15\x02\x02\u{1b6}\u{1bb}\x05\
	\x02\x02\x02\u{1b7}\u{1b8}\x07\x0a\x02\x02\u{1b8}\u{1ba}\x05\x02\x02\x02\
	\u{1b9}\u{1b7}\x03\x02\x02\x02\u{1ba}\u{1bd}\x03\x02\x02\x02\u{1bb}\u{1b9}\
	\x03\x02\x02\x02\u{1bb}\u{1bc}\x03\x02\x02\x02\u{1bc}\u{1be}\x03\x02\x02\
	\x02\u{1bd}\u{1bb}\x03\x02\x02\x02\u{1be}\u{1bf}\x07\x1a\x02\x02\u{1bf}\
	\x37\x03\x02\x02\x02\u{1c0}\u{1c1}\x07\x61\x02\x02\u{1c1}\u{1c3}\x05\x02\
	\x02\x02\u{1c2}\u{1c4}\x05\x3c\x1f\x02\u{1c3}\u{1c2}\x03\x02\x02\x02\u{1c3}\
	\u{1c4}\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\x02\x02\x02\u{1c5}\u{1c6}\x07\
	\x1b\x02\x02\u{1c6}\x39\x03\x02\x02\x02\u{1c7}\u{1c8}\x05\x02\x02\x02\u{1c8}\
	\u{1c9}\x05\x3c\x1f\x02\u{1c9}\u{1ca}\x07\x1b\x02\x02\u{1ca}\x3b\x03\x02\
	\x02\x02\u{1cb}\u{1cc}\x07\x0d\x02\x02\u{1cc}\u{1cd}\x05\x40\x21\x02\u{1cd}\
	\x3d\x03\x02\x02\x02\u{1ce}\u{1cf}\x05\x40\x21\x02\u{1cf}\x3f\x03\x02\x02\
	\x02\u{1d0}\u{1d1}\x05\x42\x22\x02\u{1d1}\x41\x03\x02\x02\x02\u{1d2}\u{1d7}\
	\x05\x44\x23\x02\u{1d3}\u{1d4}\x07\x2a\x02\x02\u{1d4}\u{1d6}\x05\x44\x23\
	\x02\u{1d5}\u{1d3}\x03\x02\x02\x02\u{1d6}\u{1d9}\x03\x02\x02\x02\u{1d7}\
	\u{1d5}\x03\x02\x02\x02\u{1d7}\u{1d8}\x03\x02\x02\x02\u{1d8}\x43\x03\x02\
	\x02\x02\u{1d9}\u{1d7}\x03\x02\x02\x02\u{1da}\u{1df}\x05\x46\x24\x02\u{1db}\
	\u{1dc}\x07\x2b\x02\x02\u{1dc}\u{1de}\x05\x46\x24\x02\u{1dd}\u{1db}\x03\
	\x02\x02\x02\u{1de}\u{1e1}\x03\x02\x02\x02\u{1df}\u{1dd}\x03\x02\x02\x02\
	\u{1df}\u{1e0}\x03\x02\x02\x02\u{1e0}\x45\x03\x02\x02\x02\u{1e1}\u{1df}\
	\x03\x02\x02\x02\u{1e2}\u{1e7}\x05\x48\x25\x02\u{1e3}\u{1e4}\x07\x22\x02\
	\x02\u{1e4}\u{1e6}\x05\x48\x25\x02\u{1e5}\u{1e3}\x03\x02\x02\x02\u{1e6}\
	\u{1e9}\x03\x02\x02\x02\u{1e7}\u{1e5}\x03\x02\x02\x02\u{1e7}\u{1e8}\x03\
	\x02\x02\x02\u{1e8}\x47\x03\x02\x02\x02\u{1e9}\u{1e7}\x03\x02\x02\x02\u{1ea}\
	\u{1eb}\x07\x21\x02\x02\u{1eb}\u{1ef}\x05\x48\x25\x02\u{1ec}\u{1ef}\x05\
	\x4a\x26\x02\u{1ed}\u{1ef}\x05\x4c\x27\x02\u{1ee}\u{1ea}\x03\x02\x02\x02\
	\u{1ee}\u{1ec}\x03\x02\x02\x02\u{1ee}\u{1ed}\x03\x02\x02\x02\u{1ef}\x49\
	\x03\x02\x02\x02\u{1f0}\u{1f4}\x05\x4e\x28\x02\u{1f1}\u{1f4}\x05\x50\x29\
	\x02\u{1f2}\u{1f4}\x05\x5e\x30\x02\u{1f3}\u{1f0}\x03\x02\x02\x02\u{1f3}\
	\u{1f1}\x03\x02\x02\x02\u{1f3}\u{1f2}\x03\x02\x02\x02\u{1f4}\x4b\x03\x02\
	\x02\x02\u{1f5}\u{1f6}\x07\x15\x02\x02\u{1f6}\u{1f7}\x05\x40\x21\x02\u{1f7}\
	\u{1f8}\x07\x1a\x02\x02\u{1f8}\x4d\x03\x02\x02\x02\u{1f9}\u{1fb}\x07\x32\
	\x02\x02\u{1fa}\u{1fc}\x05\x5a\x2e\x02\u{1fb}\u{1fa}\x03\x02\x02\x02\u{1fb}\
	\u{1fc}\x03\x02\x02\x02\u{1fc}\u{202}\x03\x02\x02\x02\u{1fd}\u{1ff}\x07\
	\x33\x02\x02\u{1fe}\u{200}\x05\x5a\x2e\x02\u{1ff}\u{1fe}\x03\x02\x02\x02\
	\u{1ff}\u{200}\x03\x02\x02\x02\u{200}\u{202}\x03\x02\x02\x02\u{201}\u{1f9}\
	\x03\x02\x02\x02\u{201}\u{1fd}\x03\x02\x02\x02\u{201}\u{202}\x03\x02\x02\
	\x02\u{202}\u{203}\x03\x02\x02\x02\u{203}\u{20d}\x05\x02\x02\x02\u{204}\
	\u{205}\x07\x22\x02\x02\u{205}\u{206}\x07\x11\x02\x02\u{206}\u{209}\x05\
	\x5a\x2e\x02\u{207}\u{208}\x07\x07\x02\x02\u{208}\u{20a}\x05\x5a\x2e\x02\
	\u{209}\u{207}\x03\x02\x02\x02\u{209}\u{20a}\x03\x02\x02\x02\u{20a}\u{20b}\
	\x03\x02\x02\x02\u{20b}\u{20c}\x07\x18\x02\x02\u{20c}\u{20e}\x03\x02\x02\
	\x02\u{20d}\u{204}\x03\x02\x02\x02\u{20d}\u{20e}\x03\x02\x02\x02\u{20e}\
	\u{229}\x03\x02\x02\x02\u{20f}\u{211}\x07\x32\x02\x02\u{210}\u{212}\x05\
	\x5a\x2e\x02\u{211}\u{210}\x03\x02\x02\x02\u{211}\u{212}\x03\x02\x02\x02\
	\u{212}\u{218}\x03\x02\x02\x02\u{213}\u{215}\x07\x33\x02\x02\u{214}\u{216}\
	\x05\x5a\x2e\x02\u{215}\u{214}\x03\x02\x02\x02\u{215}\u{216}\x03\x02\x02\
	\x02\u{216}\u{218}\x03\x02\x02\x02\u{217}\u{20f}\x03\x02\x02\x02\u{217}\
	\u{213}\x03\x02\x02\x02\u{217}\u{218}\x03\x02\x02\x02\u{218}\u{219}\x03\
	\x02\x02\x02\u{219}\u{21a}\x07\x15\x02\x02\u{21a}\u{224}\x05\x02\x02\x02\
	\u{21b}\u{21c}\x07\x22\x02\x02\u{21c}\u{21d}\x07\x11\x02\x02\u{21d}\u{220}\
	\x05\x5a\x2e\x02\u{21e}\u{21f}\x07\x07\x02\x02\u{21f}\u{221}\x05\x5a\x2e\
	\x02\u{220}\u{21e}\x03\x02\x02\x02\u{220}\u{221}\x03\x02\x02\x02\u{221}\
	\u{222}\x03\x02\x02\x02\u{222}\u{223}\x07\x18\x02\x02\u{223}\u{225}\x03\
	\x02\x02\x02\u{224}\u{21b}\x03\x02\x02\x02\u{224}\u{225}\x03\x02\x02\x02\
	\u{225}\u{226}\x03\x02\x02\x02\u{226}\u{227}\x07\x1a\x02\x02\u{227}\u{229}\
	\x03\x02\x02\x02\u{228}\u{201}\x03\x02\x02\x02\u{228}\u{217}\x03\x02\x02\
	\x02\u{229}\x4f\x03\x02\x02\x02\u{22a}\u{23a}\x05\x02\x02\x02\u{22b}\u{237}\
	\x05\x52\x2a\x02\u{22c}\u{22d}\x07\x12\x02\x02\u{22d}\u{232}\x05\x02\x02\
	\x02\u{22e}\u{22f}\x07\x0a\x02\x02\u{22f}\u{231}\x05\x02\x02\x02\u{230}\
	\u{22e}\x03\x02\x02\x02\u{231}\u{234}\x03\x02\x02\x02\u{232}\u{230}\x03\
	\x02\x02\x02\u{232}\u{233}\x03\x02\x02\x02\u{233}\u{235}\x03\x02\x02\x02\
	\u{234}\u{232}\x03\x02\x02\x02\u{235}\u{236}\x07\x19\x02\x02\u{236}\u{238}\
	\x03\x02\x02\x02\u{237}\u{22c}\x03\x02\x02\x02\u{237}\u{238}\x03\x02\x02\
	\x02\u{238}\u{23a}\x03\x02\x02\x02\u{239}\u{22a}\x03\x02\x02\x02\u{239}\
	\u{22b}\x03\x02\x02\x02\u{23a}\u{24d}\x03\x02\x02\x02\u{23b}\u{24b}\x07\
	\x03\x02\x02\u{23c}\u{24c}\x05\x02\x02\x02\u{23d}\u{249}\x05\x52\x2a\x02\
	\u{23e}\u{23f}\x07\x12\x02\x02\u{23f}\u{244}\x05\x02\x02\x02\u{240}\u{241}\
	\x07\x0a\x02\x02\u{241}\u{243}\x05\x02\x02\x02\u{242}\u{240}\x03\x02\x02\
	\x02\u{243}\u{246}\x03\x02\x02\x02\u{244}\u{242}\x03\x02\x02\x02\u{244}\
	\u{245}\x03\x02\x02\x02\u{245}\u{247}\x03\x02\x02\x02\u{246}\u{244}\x03\
	\x02\x02\x02\u{247}\u{248}\x07\x19\x02\x02\u{248}\u{24a}\x03\x02\x02\x02\
	\u{249}\u{23e}\x03\x02\x02\x02\u{249}\u{24a}\x03\x02\x02\x02\u{24a}\u{24c}\
	\x03\x02\x02\x02\u{24b}\u{23c}\x03\x02\x02\x02\u{24b}\u{23d}\x03\x02\x02\
	\x02\u{24c}\u{24e}\x03\x02\x02\x02\u{24d}\u{23b}\x03\x02\x02\x02\u{24e}\
	\u{24f}\x03\x02\x02\x02\u{24f}\u{24d}\x03\x02\x02\x02\u{24f}\u{250}\x03\
	\x02\x02\x02\u{250}\u{291}\x03\x02\x02\x02\u{251}\u{253}\x07\x32\x02\x02\
	\u{252}\u{254}\x05\x5a\x2e\x02\u{253}\u{252}\x03\x02\x02\x02\u{253}\u{254}\
	\x03\x02\x02\x02\u{254}\u{25a}\x03\x02\x02\x02\u{255}\u{257}\x07\x33\x02\
	\x02\u{256}\u{258}\x05\x5a\x2e\x02\u{257}\u{256}\x03\x02\x02\x02\u{257}\
	\u{258}\x03\x02\x02\x02\u{258}\u{25a}\x03\x02\x02\x02\u{259}\u{251}\x03\
	\x02\x02\x02\u{259}\u{255}\x03\x02\x02\x02\u{259}\u{25a}\x03\x02\x02\x02\
	\u{25a}\u{25b}\x03\x02\x02\x02\u{25b}\u{26b}\x07\x15\x02\x02\u{25c}\u{26c}\
	\x05\x02\x02\x02\u{25d}\u{269}\x05\x52\x2a\x02\u{25e}\u{25f}\x07\x12\x02\
	\x02\u{25f}\u{264}\x05\x02\x02\x02\u{260}\u{261}\x07\x0a\x02\x02\u{261}\
	\u{263}\x05\x02\x02\x02\u{262}\u{260}\x03\x02\x02\x02\u{263}\u{266}\x03\
	\x02\x02\x02\u{264}\u{262}\x03\x02\x02\x02\u{264}\u{265}\x03\x02\x02\x02\
	\u{265}\u{267}\x03\x02\x02\x02\u{266}\u{264}\x03\x02\x02\x02\u{267}\u{268}\
	\x07\x19\x02\x02\u{268}\u{26a}\x03\x02\x02\x02\u{269}\u{25e}\x03\x02\x02\
	\x02\u{269}\u{26a}\x03\x02\x02\x02\u{26a}\u{26c}\x03\x02\x02\x02\u{26b}\
	\u{25c}\x03\x02\x02\x02\u{26b}\u{25d}\x03\x02\x02\x02\u{26c}\u{27f}\x03\
	\x02\x02\x02\u{26d}\u{27d}\x07\x03\x02\x02\u{26e}\u{27e}\x05\x02\x02\x02\
	\u{26f}\u{27b}\x05\x52\x2a\x02\u{270}\u{271}\x07\x12\x02\x02\u{271}\u{276}\
	\x05\x02\x02\x02\u{272}\u{273}\x07\x0a\x02\x02\u{273}\u{275}\x05\x02\x02\
	\x02\u{274}\u{272}\x03\x02\x02\x02\u{275}\u{278}\x03\x02\x02\x02\u{276}\
	\u{274}\x03\x02\x02\x02\u{276}\u{277}\x03\x02\x02\x02\u{277}\u{279}\x03\
	\x02\x02\x02\u{278}\u{276}\x03\x02\x02\x02\u{279}\u{27a}\x07\x19\x02\x02\
	\u{27a}\u{27c}\x03\x02\x02\x02\u{27b}\u{270}\x03\x02\x02\x02\u{27b}\u{27c}\
	\x03\x02\x02\x02\u{27c}\u{27e}\x03\x02\x02\x02\u{27d}\u{26e}\x03\x02\x02\
	\x02\u{27d}\u{26f}\x03\x02\x02\x02\u{27e}\u{280}\x03\x02\x02\x02\u{27f}\
	\u{26d}\x03\x02\x02\x02\u{280}\u{281}\x03\x02\x02\x02\u{281}\u{27f}\x03\
	\x02\x02\x02\u{281}\u{282}\x03\x02\x02\x02\u{282}\u{283}\x03\x02\x02\x02\
	\u{283}\u{28e}\x07\x1a\x02\x02\u{284}\u{285}\x07\x22\x02\x02\u{285}\u{286}\
	\x07\x11\x02\x02\u{286}\u{287}\x05\x5a\x2e\x02\u{287}\u{28a}\x03\x02\x02\
	\x02\u{288}\u{289}\x07\x07\x02\x02\u{289}\u{28b}\x05\x5a\x2e\x02\u{28a}\
	\u{288}\x03\x02\x02\x02\u{28a}\u{28b}\x03\x02\x02\x02\u{28b}\u{28c}\x03\
	\x02\x02\x02\u{28c}\u{28d}\x07\x18\x02\x02\u{28d}\u{28f}\x03\x02\x02\x02\
	\u{28e}\u{284}\x03\x02\x02\x02\u{28e}\u{28f}\x03\x02\x02\x02\u{28f}\u{291}\
	\x03\x02\x02\x02\u{290}\u{239}\x03\x02\x02\x02\u{290}\u{259}\x03\x02\x02\
	\x02\u{291}\x51\x03\x02\x02\x02\u{292}\u{293}\x07\x23\x02\x02\u{293}\x53\
	\x03\x02\x02\x02\u{294}\u{295}\x07\x75\x02\x02\u{295}\x55\x03\x02\x02\x02\
	\u{296}\u{297}\x09\x0c\x02\x02\u{297}\x57\x03\x02\x02\x02\u{298}\u{2a0}\
	\x05\x5a\x2e\x02\u{299}\u{2a0}\x05\x5c\x2f\x02\u{29a}\u{2a0}\x05\x5e\x30\
	\x02\u{29b}\u{2a0}\x05\x60\x31\x02\u{29c}\u{2a0}\x05\x62\x32\x02\u{29d}\
	\u{2a0}\x05\x66\x34\x02\u{29e}\u{2a0}\x05\x64\x33\x02\u{29f}\u{298}\x03\
	\x02\x02\x02\u{29f}\u{299}\x03\x02\x02\x02\u{29f}\u{29a}\x03\x02\x02\x02\
	\u{29f}\u{29b}\x03\x02\x02\x02\u{29f}\u{29c}\x03\x02\x02\x02\u{29f}\u{29d}\
	\x03\x02\x02\x02\u{29f}\u{29e}\x03\x02\x02\x02\u{2a0}\x59\x03\x02\x02\x02\
	\u{2a1}\u{2a2}\x07\x71\x02\x02\u{2a2}\x5b\x03\x02\x02\x02\u{2a3}\u{2a4}\
	\x07\x73\x02\x02\u{2a4}\x5d\x03\x02\x02\x02\u{2a5}\u{2a6}\x07\x76\x02\x02\
	\u{2a6}\x5f\x03\x02\x02\x02\u{2a7}\u{2a8}\x07\x75\x02\x02\u{2a8}\x61\x03\
	\x02\x02\x02\u{2a9}\u{2aa}\x07\x74\x02\x02\u{2aa}\x63\x03\x02\x02\x02\u{2ab}\
	\u{2ac}\x07\x72\x02\x02\u{2ac}\x65\x03\x02\x02\x02\u{2ad}\u{2ae}\x07\x77\
	\x02\x02\u{2ae}\x67\x03\x02\x02\x02\u{2af}\u{2b0}\x07\x46\x02\x02\u{2b0}\
	\u{2b1}\x05\x02\x02\x02\u{2b1}\u{2b2}\x05\x6a\x36\x02\u{2b2}\u{2b3}\x07\
	\x1b\x02\x02\u{2b3}\x69\x03\x02\x02\x02\u{2b4}\u{2b5}\x07\x11\x02\x02\u{2b5}\
	\u{2b6}\x05\x6c\x37\x02\u{2b6}\u{2b7}\x07\x18\x02\x02\u{2b7}\x6b\x03\x02\
	\x02\x02\u{2b8}\u{2ba}\x05\x6e\x38\x02\u{2b9}\u{2b8}\x03\x02\x02\x02\u{2ba}\
	\u{2bb}\x03\x02\x02\x02\u{2bb}\u{2b9}\x03\x02\x02\x02\u{2bb}\u{2bc}\x03\
	\x02\x02\x02\u{2bc}\x6d\x03\x02\x02\x02\u{2bd}\u{2be}\x05\x7a\x3e\x02\u{2be}\
	\u{2bf}\x05\u{86}\x44\x02\u{2bf}\u{2c0}\x07\x1b\x02\x02\u{2c0}\x6f\x03\x02\
	\x02\x02\u{2c1}\u{2c2}\x07\x47\x02\x02\u{2c2}\u{2c3}\x05\x7a\x3e\x02\u{2c3}\
	\u{2c8}\x05\x72\x3a\x02\u{2c4}\u{2c5}\x07\x0a\x02\x02\u{2c5}\u{2c7}\x05\
	\x72\x3a\x02\u{2c6}\u{2c4}\x03\x02\x02\x02\u{2c7}\u{2ca}\x03\x02\x02\x02\
	\u{2c8}\u{2c6}\x03\x02\x02\x02\u{2c8}\u{2c9}\x03\x02\x02\x02\u{2c9}\u{2cb}\
	\x03\x02\x02\x02\u{2ca}\u{2c8}\x03\x02\x02\x02\u{2cb}\u{2cc}\x07\x1b\x02\
	\x02\u{2cc}\x71\x03\x02\x02\x02\u{2cd}\u{2ce}\x05\x02\x02\x02\u{2ce}\u{2cf}\
	\x07\x0d\x02\x02\u{2cf}\u{2d0}\x05\u{8a}\x46\x02\u{2d0}\x73\x03\x02\x02\
	\x02\u{2d1}\u{2d2}\x05\x7a\x3e\x02\u{2d2}\u{2d7}\x05\u{86}\x44\x02\u{2d3}\
	\u{2d4}\x07\x0a\x02\x02\u{2d4}\u{2d6}\x05\u{86}\x44\x02\u{2d5}\u{2d3}\x03\
	\x02\x02\x02\u{2d6}\u{2d9}\x03\x02\x02\x02\u{2d7}\u{2d5}\x03\x02\x02\x02\
	\u{2d7}\u{2d8}\x03\x02\x02\x02\u{2d8}\u{2da}\x03\x02\x02\x02\u{2d9}\u{2d7}\
	\x03\x02\x02\x02\u{2da}\u{2db}\x07\x1b\x02\x02\u{2db}\x75\x03\x02\x02\x02\
	\u{2dc}\u{2dd}\x05\x7a\x3e\x02\u{2dd}\u{2e2}\x05\u{86}\x44\x02\u{2de}\u{2df}\
	\x07\x0a\x02\x02\u{2df}\u{2e1}\x05\u{86}\x44\x02\u{2e0}\u{2de}\x03\x02\x02\
	\x02\u{2e1}\u{2e4}\x03\x02\x02\x02\u{2e2}\u{2e0}\x03\x02\x02\x02\u{2e2}\
	\u{2e3}\x03\x02\x02\x02\u{2e3}\u{2e5}\x03\x02\x02\x02\u{2e4}\u{2e2}\x03\
	\x02\x02\x02\u{2e5}\u{2e6}\x07\x1b\x02\x02\u{2e6}\x77\x03\x02\x02\x02\u{2e7}\
	\u{2e8}\x09\x0d\x02\x02\u{2e8}\x79\x03\x02\x02\x02\u{2e9}\u{2ed}\x05\u{80}\
	\x41\x02\u{2ea}\u{2ed}\x05\u{82}\x42\x02\u{2eb}\u{2ed}\x05\x7e\x40\x02\u{2ec}\
	\u{2e9}\x03\x02\x02\x02\u{2ec}\u{2ea}\x03\x02\x02\x02\u{2ec}\u{2eb}\x03\
	\x02\x02\x02\u{2ed}\x7b\x03\x02\x02\x02\u{2ee}\u{2f1}\x05\u{80}\x41\x02\
	\u{2ef}\u{2f1}\x05\x7e\x40\x02\u{2f0}\u{2ee}\x03\x02\x02\x02\u{2f0}\u{2ef}\
	\x03\x02\x02\x02\u{2f1}\x7d\x03\x02\x02\x02\u{2f2}\u{2f3}\x07\x70\x02\x02\
	\u{2f3}\u{2f4}\x07\x12\x02\x02\u{2f4}\u{2f5}\x07\x71\x02\x02\u{2f5}\u{2f6}\
	\x07\x19\x02\x02\u{2f6}\x7f\x03\x02\x02\x02\u{2f7}\u{2f8}\x09\x0e\x02\x02\
	\u{2f8}\u{81}\x03\x02\x02\x02\u{2f9}\u{2fa}\x07\x50\x02\x02\u{2fa}\u{2fb}\
	\x07\x11\x02\x02\u{2fb}\u{300}\x05\u{84}\x43\x02\u{2fc}\u{2fd}\x07\x0a\x02\
	\x02\u{2fd}\u{2ff}\x05\u{84}\x43\x02\u{2fe}\u{2fc}\x03\x02\x02\x02\u{2ff}\
	\u{302}\x03\x02\x02\x02\u{300}\u{2fe}\x03\x02\x02\x02\u{300}\u{301}\x03\
	\x02\x02\x02\u{301}\u{303}\x03\x02\x02\x02\u{302}\u{300}\x03\x02\x02\x02\
	\u{303}\u{304}\x07\x18\x02\x02\u{304}\u{83}\x03\x02\x02\x02\u{305}\u{306}\
	\x05\x02\x02\x02\u{306}\u{85}\x03\x02\x02\x02\u{307}\u{30a}\x05\x02\x02\
	\x02\u{308}\u{309}\x07\x0d\x02\x02\u{309}\u{30b}\x05\u{8a}\x46\x02\u{30a}\
	\u{308}\x03\x02\x02\x02\u{30a}\u{30b}\x03\x02\x02\x02\u{30b}\u{30d}\x03\
	\x02\x02\x02\u{30c}\u{30e}\x05\u{88}\x45\x02\u{30d}\u{30c}\x03\x02\x02\x02\
	\u{30d}\u{30e}\x03\x02\x02\x02\u{30e}\u{87}\x03\x02\x02\x02\u{30f}\u{310}\
	\x07\x51\x02\x02\u{310}\u{311}\x05\u{92}\x4a\x02\u{311}\u{89}\x03\x02\x02\
	\x02\u{312}\u{313}\x05\u{92}\x4a\x02\u{313}\u{8b}\x03\x02\x02\x02\u{314}\
	\u{316}\x05\u{cc}\x67\x02\u{315}\u{314}\x03\x02\x02\x02\u{315}\u{316}\x03\
	\x02\x02\x02\u{316}\u{317}\x03\x02\x02\x02\u{317}\u{318}\x07\x63\x02\x02\
	\u{318}\u{31a}\x05\u{8e}\x48\x02\u{319}\u{31b}\x05\x26\x14\x02\u{31a}\u{319}\
	\x03\x02\x02\x02\u{31a}\u{31b}\x03\x02\x02\x02\u{31b}\u{31c}\x03\x02\x02\
	\x02\u{31c}\u{31d}\x07\x1b\x02\x02\u{31d}\u{8d}\x03\x02\x02\x02\u{31e}\u{320}\
	\x09\x0f\x02\x02\u{31f}\u{31e}\x03\x02\x02\x02\u{31f}\u{320}\x03\x02\x02\
	\x02\u{320}\u{321}\x03\x02\x02\x02\u{321}\u{322}\x05\u{92}\x4a\x02\u{322}\
	\u{8f}\x03\x02\x02\x02\u{323}\u{324}\x05\u{92}\x4a\x02\u{324}\u{325}\x07\
	\x1b\x02\x02\u{325}\u{91}\x03\x02\x02\x02\u{326}\u{329}\x05\u{94}\x4b\x02\
	\u{327}\u{328}\x09\x10\x02\x02\u{328}\u{32a}\x05\u{92}\x4a\x02\u{329}\u{327}\
	\x03\x02\x02\x02\u{329}\u{32a}\x03\x02\x02\x02\u{32a}\u{93}\x03\x02\x02\
	\x02\u{32b}\u{330}\x05\u{96}\x4c\x02\u{32c}\u{32d}\x07\x20\x02\x02\u{32d}\
	\u{32f}\x05\u{96}\x4c\x02\u{32e}\u{32c}\x03\x02\x02\x02\u{32f}\u{332}\x03\
	\x02\x02\x02\u{330}\u{32e}\x03\x02\x02\x02\u{330}\u{331}\x03\x02\x02\x02\
	\u{331}\u{95}\x03\x02\x02\x02\u{332}\u{330}\x03\x02\x02\x02\u{333}\u{338}\
	\x05\u{98}\x4d\x02\u{334}\u{335}\x07\x2a\x02\x02\u{335}\u{337}\x05\u{98}\
	\x4d\x02\u{336}\u{334}\x03\x02\x02\x02\u{337}\u{33a}\x03\x02\x02\x02\u{338}\
	\u{336}\x03\x02\x02\x02\u{338}\u{339}\x03\x02\x02\x02\u{339}\u{97}\x03\x02\
	\x02\x02\u{33a}\u{338}\x03\x02\x02\x02\u{33b}\u{340}\x05\u{9a}\x4e\x02\u{33c}\
	\u{33d}\x07\x2b\x02\x02\u{33d}\u{33f}\x05\u{9a}\x4e\x02\u{33e}\u{33c}\x03\
	\x02\x02\x02\u{33f}\u{342}\x03\x02\x02\x02\u{340}\u{33e}\x03\x02\x02\x02\
	\u{340}\u{341}\x03\x02\x02\x02\u{341}\u{99}\x03\x02\x02\x02\u{342}\u{340}\
	\x03\x02\x02\x02\u{343}\u{348}\x05\u{9c}\x4f\x02\u{344}\u{345}\x07\x22\x02\
	\x02\u{345}\u{347}\x05\u{9c}\x4f\x02\u{346}\u{344}\x03\x02\x02\x02\u{347}\
	\u{34a}\x03\x02\x02\x02\u{348}\u{346}\x03\x02\x02\x02\u{348}\u{349}\x03\
	\x02\x02\x02\u{349}\u{9b}\x03\x02\x02\x02\u{34a}\u{348}\x03\x02\x02\x02\
	\u{34b}\u{350}\x05\u{9e}\x50\x02\u{34c}\u{34d}\x07\x06\x02\x02\u{34d}\u{34f}\
	\x05\u{9e}\x50\x02\u{34e}\u{34c}\x03\x02\x02\x02\u{34f}\u{352}\x03\x02\x02\
	\x02\u{350}\u{34e}\x03\x02\x02\x02\u{350}\u{351}\x03\x02\x02\x02\u{351}\
	\u{9d}\x03\x02\x02\x02\u{352}\u{350}\x03\x02\x02\x02\u{353}\u{358}\x05\u{a0}\
	\x51\x02\u{354}\u{355}\x07\x24\x02\x02\u{355}\u{357}\x05\u{a0}\x51\x02\u{356}\
	\u{354}\x03\x02\x02\x02\u{357}\u{35a}\x03\x02\x02\x02\u{358}\u{356}\x03\
	\x02\x02\x02\u{358}\u{359}\x03\x02\x02\x02\u{359}\u{9f}\x03\x02\x02\x02\
	\u{35a}\u{358}\x03\x02\x02\x02\u{35b}\u{360}\x05\u{a2}\x52\x02\u{35c}\u{35d}\
	\x09\x11\x02\x02\u{35d}\u{35f}\x05\u{a2}\x52\x02\u{35e}\u{35c}\x03\x02\x02\
	\x02\u{35f}\u{362}\x03\x02\x02\x02\u{360}\u{35e}\x03\x02\x02\x02\u{360}\
	\u{361}\x03\x02\x02\x02\u{361}\u{a1}\x03\x02\x02\x02\u{362}\u{360}\x03\x02\
	\x02\x02\u{363}\u{368}\x05\u{a4}\x53\x02\u{364}\u{365}\x09\x12\x02\x02\u{365}\
	\u{367}\x05\u{a4}\x53\x02\u{366}\u{364}\x03\x02\x02\x02\u{367}\u{36a}\x03\
	\x02\x02\x02\u{368}\u{366}\x03\x02\x02\x02\u{368}\u{369}\x03\x02\x02\x02\
	\u{369}\u{a3}\x03\x02\x02\x02\u{36a}\u{368}\x03\x02\x02\x02\u{36b}\u{370}\
	\x05\u{a6}\x54\x02\u{36c}\u{36d}\x09\x13\x02\x02\u{36d}\u{36f}\x05\u{a6}\
	\x54\x02\u{36e}\u{36c}\x03\x02\x02\x02\u{36f}\u{372}\x03\x02\x02\x02\u{370}\
	\u{36e}\x03\x02\x02\x02\u{370}\u{371}\x03\x02\x02\x02\u{371}\u{a5}\x03\x02\
	\x02\x02\u{372}\u{370}\x03\x02\x02\x02\u{373}\u{378}\x05\u{a8}\x55\x02\u{374}\
	\u{375}\x09\x14\x02\x02\u{375}\u{377}\x05\u{a8}\x55\x02\u{376}\u{374}\x03\
	\x02\x02\x02\u{377}\u{37a}\x03\x02\x02\x02\u{378}\u{376}\x03\x02\x02\x02\
	\u{378}\u{379}\x03\x02\x02\x02\u{379}\u{a7}\x03\x02\x02\x02\u{37a}\u{378}\
	\x03\x02\x02\x02\u{37b}\u{380}\x05\u{aa}\x56\x02\u{37c}\u{37d}\x09\x15\x02\
	\x02\u{37d}\u{37f}\x05\u{aa}\x56\x02\u{37e}\u{37c}\x03\x02\x02\x02\u{37f}\
	\u{382}\x03\x02\x02\x02\u{380}\u{37e}\x03\x02\x02\x02\u{380}\u{381}\x03\
	\x02\x02\x02\u{381}\u{a9}\x03\x02\x02\x02\u{382}\u{380}\x03\x02\x02\x02\
	\u{383}\u{388}\x05\u{ac}\x57\x02\u{384}\u{385}\x07\x28\x02\x02\u{385}\u{387}\
	\x05\u{ac}\x57\x02\u{386}\u{384}\x03\x02\x02\x02\u{387}\u{38a}\x03\x02\x02\
	\x02\u{388}\u{386}\x03\x02\x02\x02\u{388}\u{389}\x03\x02\x02\x02\u{389}\
	\u{ab}\x03\x02\x02\x02\u{38a}\u{388}\x03\x02\x02\x02\u{38b}\u{38c}\x07\x1e\
	\x02\x02\u{38c}\u{391}\x05\u{ac}\x57\x02\u{38d}\u{38e}\x07\x16\x02\x02\u{38e}\
	\u{391}\x05\u{ac}\x57\x02\u{38f}\u{391}\x05\u{ae}\x58\x02\u{390}\u{38b}\
	\x03\x02\x02\x02\u{390}\u{38d}\x03\x02\x02\x02\u{390}\u{38f}\x03\x02\x02\
	\x02\u{391}\u{ad}\x03\x02\x02\x02\u{392}\u{393}\x07\x21\x02\x02\u{393}\u{39b}\
	\x05\u{ac}\x57\x02\u{394}\u{395}\x07\x25\x02\x02\u{395}\u{39b}\x05\u{ac}\
	\x57\x02\u{396}\u{398}\x05\u{c2}\x62\x02\u{397}\u{399}\x09\x16\x02\x02\u{398}\
	\u{397}\x03\x02\x02\x02\u{398}\u{399}\x03\x02\x02\x02\u{399}\u{39b}\x03\
	\x02\x02\x02\u{39a}\u{392}\x03\x02\x02\x02\u{39a}\u{394}\x03\x02\x02\x02\
	\u{39a}\u{396}\x03\x02\x02\x02\u{39b}\u{af}\x03\x02\x02\x02\u{39c}\u{39d}\
	\x07\x6a\x02\x02\u{39d}\u{39e}\x07\x15\x02\x02\u{39e}\u{3a1}\x05\u{92}\x4a\
	\x02\u{39f}\u{3a0}\x07\x0a\x02\x02\u{3a0}\u{3a2}\x05\u{92}\x4a\x02\u{3a1}\
	\u{39f}\x03\x02\x02\x02\u{3a2}\u{3a3}\x03\x02\x02\x02\u{3a3}\u{3a1}\x03\
	\x02\x02\x02\u{3a3}\u{3a4}\x03\x02\x02\x02\u{3a4}\u{3a5}\x03\x02\x02\x02\
	\u{3a5}\u{3a6}\x07\x1a\x02\x02\u{3a6}\u{b1}\x03\x02\x02\x02\u{3a7}\u{3a8}\
	\x07\x66\x02\x02\u{3a8}\u{3a9}\x07\x15\x02\x02\u{3a9}\u{3aa}\x05\x02\x02\
	\x02\u{3aa}\u{3ab}\x07\x1a\x02\x02\u{3ab}\u{b3}\x03\x02\x02\x02\u{3ac}\u{3ad}\
	\x07\x64\x02\x02\u{3ad}\u{3ae}\x07\x15\x02\x02\u{3ae}\u{3af}\x05\u{c4}\x63\
	\x02\u{3af}\u{3b0}\x07\x1a\x02\x02\u{3b0}\u{b5}\x03\x02\x02\x02\u{3b1}\u{3b2}\
	\x07\x56\x02\x02\u{3b2}\u{3b3}\x07\x15\x02\x02\u{3b3}\u{3b4}\x05\x02\x02\
	\x02\u{3b4}\u{3b5}\x07\x1a\x02\x02\u{3b5}\u{b7}\x03\x02\x02\x02\u{3b6}\u{3b7}\
	\x07\x6c\x02\x02\u{3b7}\u{3b8}\x05\x02\x02\x02\u{3b8}\u{3b9}\x07\x07\x02\
	\x02\u{3b9}\u{3ba}\x05\x7c\x3f\x02\u{3ba}\u{3bb}\x05\u{ba}\x5e\x02\u{3bb}\
	\u{b9}\x03\x02\x02\x02\u{3bc}\u{3bd}\x05\u{bc}\x5f\x02\u{3bd}\u{3c1}\x07\
	\x11\x02\x02\u{3be}\u{3c0}\x05\x76\x3c\x02\u{3bf}\u{3be}\x03\x02\x02\x02\
	\u{3c0}\u{3c3}\x03\x02\x02\x02\u{3c1}\u{3bf}\x03\x02\x02\x02\u{3c1}\u{3c2}\
	\x03\x02\x02\x02\u{3c2}\u{3c5}\x03\x02\x02\x02\u{3c3}\u{3c1}\x03\x02\x02\
	\x02\u{3c4}\u{3c6}\x05\u{90}\x49\x02\u{3c5}\u{3c4}\x03\x02\x02\x02\u{3c6}\
	\u{3c7}\x03\x02\x02\x02\u{3c7}\u{3c5}\x03\x02\x02\x02\u{3c7}\u{3c8}\x03\
	\x02\x02\x02\u{3c8}\u{3c9}\x03\x02\x02\x02\u{3c9}\u{3ca}\x07\x18\x02\x02\
	\u{3ca}\u{bb}\x03\x02\x02\x02\u{3cb}\u{3cd}\x07\x15\x02\x02\u{3cc}\u{3ce}\
	\x05\u{be}\x60\x02\u{3cd}\u{3cc}\x03\x02\x02\x02\u{3cd}\u{3ce}\x03\x02\x02\
	\x02\u{3ce}\u{3d3}\x03\x02\x02\x02\u{3cf}\u{3d0}\x07\x0a\x02\x02\u{3d0}\
	\u{3d2}\x05\u{be}\x60\x02\u{3d1}\u{3cf}\x03\x02\x02\x02\u{3d2}\u{3d5}\x03\
	\x02\x02\x02\u{3d3}\u{3d1}\x03\x02\x02\x02\u{3d3}\u{3d4}\x03\x02\x02\x02\
	\u{3d4}\u{3d6}\x03\x02\x02\x02\u{3d5}\u{3d3}\x03\x02\x02\x02\u{3d6}\u{3d7}\
	\x07\x1a\x02\x02\u{3d7}\u{bd}\x03\x02\x02\x02\u{3d8}\u{3d9}\x05\x02\x02\
	\x02\u{3d9}\u{3da}\x07\x07\x02\x02\u{3da}\u{3db}\x05\x7c\x3f\x02\u{3db}\
	\u{bf}\x03\x02\x02\x02\u{3dc}\u{3dd}\x07\x6d\x02\x02\u{3dd}\u{3de}\x05\u{92}\
	\x4a\x02\u{3de}\u{c1}\x03\x02\x02\x02\u{3df}\u{3e8}\x05\u{c6}\x64\x02\u{3e0}\
	\u{3e8}\x05\u{c4}\x63\x02\u{3e1}\u{3e8}\x05\x58\x2d\x02\u{3e2}\u{3e8}\x05\
	\u{b4}\x5b\x02\u{3e3}\u{3e8}\x05\u{b2}\x5a\x02\u{3e4}\u{3e8}\x05\u{b0}\x59\
	\x02\u{3e5}\u{3e8}\x05\u{c0}\x61\x02\u{3e6}\u{3e8}\x05\u{c8}\x65\x02\u{3e7}\
	\u{3df}\x03\x02\x02\x02\u{3e7}\u{3e0}\x03\x02\x02\x02\u{3e7}\u{3e1}\x03\
	\x02\x02\x02\u{3e7}\u{3e2}\x03\x02\x02\x02\u{3e7}\u{3e3}\x03\x02\x02\x02\
	\u{3e7}\u{3e4}\x03\x02\x02\x02\u{3e7}\u{3e5}\x03\x02\x02\x02\u{3e7}\u{3e6}\
	\x03\x02\x02\x02\u{3e8}\u{c3}\x03\x02\x02\x02\u{3e9}\u{3ec}\x05\x02\x02\
	\x02\u{3ea}\u{3eb}\x07\x0b\x02\x02\u{3eb}\u{3ed}\x05\x02\x02\x02\u{3ec}\
	\u{3ea}\x03\x02\x02\x02\u{3ec}\u{3ed}\x03\x02\x02\x02\u{3ed}\u{c5}\x03\x02\
	\x02\x02\u{3ee}\u{3ef}\x07\x15\x02\x02\u{3ef}\u{3f0}\x05\u{92}\x4a\x02\u{3f0}\
	\u{3f1}\x07\x1a\x02\x02\u{3f1}\u{c7}\x03\x02\x02\x02\u{3f2}\u{3f3}\x05\x02\
	\x02\x02\u{3f3}\u{3f4}\x07\x15\x02\x02\u{3f4}\u{3f9}\x05\u{92}\x4a\x02\u{3f5}\
	\u{3f6}\x07\x0a\x02\x02\u{3f6}\u{3f8}\x05\u{92}\x4a\x02\u{3f7}\u{3f5}\x03\
	\x02\x02\x02\u{3f8}\u{3fb}\x03\x02\x02\x02\u{3f9}\u{3f7}\x03\x02\x02\x02\
	\u{3f9}\u{3fa}\x03\x02\x02\x02\u{3fa}\u{3fc}\x03\x02\x02\x02\u{3fb}\u{3f9}\
	\x03\x02\x02\x02\u{3fc}\u{3fd}\x07\x1a\x02\x02\u{3fd}\u{c9}\x03\x02\x02\
	\x02\u{3fe}\u{3ff}\x07\x6e\x02\x02\u{3ff}\u{400}\x05\u{c6}\x64\x02\u{400}\
	\u{403}\x05\u{90}\x49\x02\u{401}\u{402}\x07\x6f\x02\x02\u{402}\u{404}\x05\
	\u{90}\x49\x02\u{403}\u{401}\x03\x02\x02\x02\u{403}\u{404}\x03\x02\x02\x02\
	\u{404}\u{cb}\x03\x02\x02\x02\u{405}\u{406}\x07\x05\x02\x02\u{406}\u{407}\
	\x07\x49\x02\x02\u{407}\u{408}\x07\x07\x02\x02\u{408}\u{409}\x05\x02\x02\
	\x02\u{409}\u{cd}\x03\x02\x02\x02\x6d\u{db}\u{e1}\u{e6}\u{ec}\u{f1}\u{100}\
	\u{102}\u{108}\u{10e}\u{114}\u{118}\u{11f}\u{12a}\u{131}\u{13c}\u{141}\u{14c}\
	\u{151}\u{155}\u{157}\u{15f}\u{168}\u{171}\u{17f}\u{181}\u{18a}\u{18d}\u{190}\
	\u{198}\u{1a2}\u{1b1}\u{1bb}\u{1c3}\u{1d7}\u{1df}\u{1e7}\u{1ee}\u{1f3}\u{1fb}\
	\u{1ff}\u{201}\u{209}\u{20d}\u{211}\u{215}\u{217}\u{220}\u{224}\u{228}\u{232}\
	\u{237}\u{239}\u{244}\u{249}\u{24b}\u{24f}\u{253}\u{257}\u{259}\u{264}\u{269}\
	\u{26b}\u{276}\u{27b}\u{27d}\u{281}\u{28a}\u{28e}\u{290}\u{29f}\u{2bb}\u{2c8}\
	\u{2d7}\u{2e2}\u{2ec}\u{2f0}\u{300}\u{30a}\u{30d}\u{315}\u{31a}\u{31f}\u{329}\
	\u{330}\u{338}\u{340}\u{348}\u{350}\u{358}\u{360}\u{368}\u{370}\u{378}\u{380}\
	\u{388}\u{390}\u{398}\u{39a}\u{3a3}\u{3c1}\u{3c7}\u{3cd}\u{3d3}\u{3e7}\u{3ec}\
	\u{3f9}\u{403}";

