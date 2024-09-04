#![allow(nonstandard_style)]
// Generated from grammar/CycloneParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::cycloneparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CycloneParser}.
 */
pub trait CycloneParserVisitor<'input>: ParseTreeVisitor<'input,CycloneParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CycloneParser#identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#compOptions}.
	 * @param ctx the parse tree
	 */
	fn visit_compOptions(&mut self, ctx: &CompOptionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#optionName}.
	 * @param ctx the parse tree
	 */
	fn visit_optionName(&mut self, ctx: &OptionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#statementList}.
	 * @param ctx the parse tree
	 */
	fn visit_statementList(&mut self, ctx: &StatementListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transList}.
	 * @param ctx the parse tree
	 */
	fn visit_transList(&mut self, ctx: &TransListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#letOrPathAssignExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_letOrPathAssignExpr(&mut self, ctx: &LetOrPathAssignExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalDefinitions}.
	 * @param ctx the parse tree
	 */
	fn visit_globalDefinitions(&mut self, ctx: &GlobalDefinitionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#machineDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_machineDecl(&mut self, ctx: &MachineDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#machineScope}.
	 * @param ctx the parse tree
	 */
	fn visit_machineScope(&mut self, ctx: &MachineScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_stateExpr(&mut self, ctx: &StateExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateScope}.
	 * @param ctx the parse tree
	 */
	fn visit_stateScope(&mut self, ctx: &StateScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#trans}.
	 * @param ctx the parse tree
	 */
	fn visit_trans(&mut self, ctx: &TransContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transScope}.
	 * @param ctx the parse tree
	 */
	fn visit_transScope(&mut self, ctx: &TransScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transOp}.
	 * @param ctx the parse tree
	 */
	fn visit_transOp(&mut self, ctx: &TransOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transDef}.
	 * @param ctx the parse tree
	 */
	fn visit_transDef(&mut self, ctx: &TransDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transExclExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_transExclExpr(&mut self, ctx: &TransExclExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#invariantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_invariantExpression(&mut self, ctx: &InvariantExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#inExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_inExpr(&mut self, ctx: &InExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#invariantScope}.
	 * @param ctx the parse tree
	 */
	fn visit_invariantScope(&mut self, ctx: &InvariantScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#goal}.
	 * @param ctx the parse tree
	 */
	fn visit_goal(&mut self, ctx: &GoalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#checkExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_checkExpr(&mut self, ctx: &CheckExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#forExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_forExpr(&mut self, ctx: &ForExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stopExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_stopExpr(&mut self, ctx: &StopExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#viaExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_viaExpr(&mut self, ctx: &ViaExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathExprList}.
	 * @param ctx the parse tree
	 */
	fn visit_pathExprList(&mut self, ctx: &PathExprListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#withExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_withExpr(&mut self, ctx: &WithExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#letExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_letExpr(&mut self, ctx: &LetExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathAssignStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_pathAssignStatement(&mut self, ctx: &PathAssignStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathCondAssignExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_pathCondAssignExpr(&mut self, ctx: &PathCondAssignExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_pathExpr(&mut self, ctx: &PathExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_pathCondition(&mut self, ctx: &PathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#orPathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_orPathCondition(&mut self, ctx: &OrPathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#andPathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_andPathCondition(&mut self, ctx: &AndPathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#xorPathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_xorPathCondition(&mut self, ctx: &XorPathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryPathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryPathCondition(&mut self, ctx: &UnaryPathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primaryCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryCondition(&mut self, ctx: &PrimaryConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#parPathCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_parPathCondition(&mut self, ctx: &ParPathConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateIncExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_stateIncExpr(&mut self, ctx: &StateIncExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathPrimaryExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_pathPrimaryExpr(&mut self, ctx: &PathPrimaryExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathOp}.
	 * @param ctx the parse tree
	 */
	fn visit_pathOp(&mut self, ctx: &PathOpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#label}.
	 * @param ctx the parse tree
	 */
	fn visit_label(&mut self, ctx: &LabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_stateModifier(&mut self, ctx: &StateModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#intLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_intLiteral(&mut self, ctx: &IntLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#realLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_realLiteral(&mut self, ctx: &RealLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#boolLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_boolLiteral(&mut self, ctx: &BoolLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#charLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_charLiteral(&mut self, ctx: &CharLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bvLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_bvLiteral(&mut self, ctx: &BvLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumLiteral(&mut self, ctx: &EnumLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#record}.
	 * @param ctx the parse tree
	 */
	fn visit_record(&mut self, ctx: &RecordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordScope}.
	 * @param ctx the parse tree
	 */
	fn visit_recordScope(&mut self, ctx: &RecordScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordVariableDeclGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_recordVariableDeclGroup(&mut self, ctx: &RecordVariableDeclGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordVariableDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_recordVariableDecl(&mut self, ctx: &RecordVariableDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalConstantGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_globalConstantGroup(&mut self, ctx: &GlobalConstantGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalConstantDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_globalConstantDecl(&mut self, ctx: &GlobalConstantDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalVariableGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_globalVariableGroup(&mut self, ctx: &GlobalVariableGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#localVariableGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_localVariableGroup(&mut self, ctx: &LocalVariableGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#modifier}.
	 * @param ctx the parse tree
	 */
	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#type_mark}.
	 * @param ctx the parse tree
	 */
	fn visit_type_mark(&mut self, ctx: &Type_markContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primitiveBvType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveBvType(&mut self, ctx: &PrimitiveBvTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bvType}.
	 * @param ctx the parse tree
	 */
	fn visit_bvType(&mut self, ctx: &BvTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumType}.
	 * @param ctx the parse tree
	 */
	fn visit_enumType(&mut self, ctx: &EnumTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDecl(&mut self, ctx: &EnumDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#variableDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#whereExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_whereExpr(&mut self, ctx: &WhereExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#variableInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#assertExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_assertExpr(&mut self, ctx: &AssertExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#assertMainExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_assertMainExpr(&mut self, ctx: &AssertMainExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalImpliesExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalImpliesExpression(&mut self, ctx: &ConditionalImpliesExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalOrExpression(&mut self, ctx: &ConditionalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalAndExpression(&mut self, ctx: &ConditionalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalXorExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalXorExpression(&mut self, ctx: &ConditionalXorExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitwiseOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitwiseAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitShiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_bitShiftExpression(&mut self, ctx: &BitShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#powExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryExpressionNotPlusMinus}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpressionNotPlusMinus(&mut self, ctx: &UnaryExpressionNotPlusMinusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#oneExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_oneExpr(&mut self, ctx: &OneExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#freshExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_freshExpr(&mut self, ctx: &FreshExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#initialExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_initialExpr(&mut self, ctx: &InitialExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#prevExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_prevExpr(&mut self, ctx: &PrevExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDeclaration(&mut self, ctx: &FunctionDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionBodyScope}.
	 * @param ctx the parse tree
	 */
	fn visit_functionBodyScope(&mut self, ctx: &FunctionBodyScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionParamsDecl}.
	 * @param ctx the parse tree
	 */
	fn visit_functionParamsDecl(&mut self, ctx: &FunctionParamsDeclContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionParam}.
	 * @param ctx the parse tree
	 */
	fn visit_functionParam(&mut self, ctx: &FunctionParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#returnExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_returnExpr(&mut self, ctx: &ReturnExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primary}.
	 * @param ctx the parse tree
	 */
	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#dotIdentifierExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_dotIdentifierExpr(&mut self, ctx: &DotIdentifierExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#parExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#funCall}.
	 * @param ctx the parse tree
	 */
	fn visit_funCall(&mut self, ctx: &FunCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#iteStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_iteStatement(&mut self, ctx: &IteStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CycloneParser#annotationExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationExpr(&mut self, ctx: &AnnotationExprContext<'input>) { self.visit_children(ctx) }

}

pub trait CycloneParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= CycloneParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CycloneParser#identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#compOptions}.
	 * @param ctx the parse tree
	 */
		fn visit_compOptions(&mut self, ctx: &CompOptionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#optionName}.
	 * @param ctx the parse tree
	 */
		fn visit_optionName(&mut self, ctx: &OptionNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#statementList}.
	 * @param ctx the parse tree
	 */
		fn visit_statementList(&mut self, ctx: &StatementListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transList}.
	 * @param ctx the parse tree
	 */
		fn visit_transList(&mut self, ctx: &TransListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#letOrPathAssignExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_letOrPathAssignExpr(&mut self, ctx: &LetOrPathAssignExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalDefinitions}.
	 * @param ctx the parse tree
	 */
		fn visit_globalDefinitions(&mut self, ctx: &GlobalDefinitionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#machineDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_machineDecl(&mut self, ctx: &MachineDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#machineScope}.
	 * @param ctx the parse tree
	 */
		fn visit_machineScope(&mut self, ctx: &MachineScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_stateExpr(&mut self, ctx: &StateExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateScope}.
	 * @param ctx the parse tree
	 */
		fn visit_stateScope(&mut self, ctx: &StateScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#trans}.
	 * @param ctx the parse tree
	 */
		fn visit_trans(&mut self, ctx: &TransContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transScope}.
	 * @param ctx the parse tree
	 */
		fn visit_transScope(&mut self, ctx: &TransScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transOp}.
	 * @param ctx the parse tree
	 */
		fn visit_transOp(&mut self, ctx: &TransOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transDef}.
	 * @param ctx the parse tree
	 */
		fn visit_transDef(&mut self, ctx: &TransDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#transExclExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_transExclExpr(&mut self, ctx: &TransExclExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#invariantExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_invariantExpression(&mut self, ctx: &InvariantExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#inExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_inExpr(&mut self, ctx: &InExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#invariantScope}.
	 * @param ctx the parse tree
	 */
		fn visit_invariantScope(&mut self, ctx: &InvariantScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#goal}.
	 * @param ctx the parse tree
	 */
		fn visit_goal(&mut self, ctx: &GoalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#checkExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_checkExpr(&mut self, ctx: &CheckExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#forExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_forExpr(&mut self, ctx: &ForExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stopExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_stopExpr(&mut self, ctx: &StopExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#viaExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_viaExpr(&mut self, ctx: &ViaExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathExprList}.
	 * @param ctx the parse tree
	 */
		fn visit_pathExprList(&mut self, ctx: &PathExprListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#withExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_withExpr(&mut self, ctx: &WithExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#letExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_letExpr(&mut self, ctx: &LetExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathAssignStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_pathAssignStatement(&mut self, ctx: &PathAssignStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathCondAssignExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_pathCondAssignExpr(&mut self, ctx: &PathCondAssignExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_pathExpr(&mut self, ctx: &PathExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_pathCondition(&mut self, ctx: &PathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#orPathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_orPathCondition(&mut self, ctx: &OrPathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#andPathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_andPathCondition(&mut self, ctx: &AndPathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#xorPathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_xorPathCondition(&mut self, ctx: &XorPathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryPathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryPathCondition(&mut self, ctx: &UnaryPathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primaryCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryCondition(&mut self, ctx: &PrimaryConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#parPathCondition}.
	 * @param ctx the parse tree
	 */
		fn visit_parPathCondition(&mut self, ctx: &ParPathConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateIncExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_stateIncExpr(&mut self, ctx: &StateIncExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathPrimaryExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_pathPrimaryExpr(&mut self, ctx: &PathPrimaryExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#pathOp}.
	 * @param ctx the parse tree
	 */
		fn visit_pathOp(&mut self, ctx: &PathOpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#label}.
	 * @param ctx the parse tree
	 */
		fn visit_label(&mut self, ctx: &LabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stateModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_stateModifier(&mut self, ctx: &StateModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#intLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_intLiteral(&mut self, ctx: &IntLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#realLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_realLiteral(&mut self, ctx: &RealLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#boolLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_boolLiteral(&mut self, ctx: &BoolLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#charLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_charLiteral(&mut self, ctx: &CharLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bvLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_bvLiteral(&mut self, ctx: &BvLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumLiteral(&mut self, ctx: &EnumLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#record}.
	 * @param ctx the parse tree
	 */
		fn visit_record(&mut self, ctx: &RecordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordScope}.
	 * @param ctx the parse tree
	 */
		fn visit_recordScope(&mut self, ctx: &RecordScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordVariableDeclGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_recordVariableDeclGroup(&mut self, ctx: &RecordVariableDeclGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#recordVariableDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_recordVariableDecl(&mut self, ctx: &RecordVariableDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalConstantGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_globalConstantGroup(&mut self, ctx: &GlobalConstantGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalConstantDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_globalConstantDecl(&mut self, ctx: &GlobalConstantDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#globalVariableGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_globalVariableGroup(&mut self, ctx: &GlobalVariableGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#localVariableGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_localVariableGroup(&mut self, ctx: &LocalVariableGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#modifier}.
	 * @param ctx the parse tree
	 */
		fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#type_mark}.
	 * @param ctx the parse tree
	 */
		fn visit_type_mark(&mut self, ctx: &Type_markContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primitiveBvType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveBvType(&mut self, ctx: &PrimitiveBvTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bvType}.
	 * @param ctx the parse tree
	 */
		fn visit_bvType(&mut self, ctx: &BvTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumType}.
	 * @param ctx the parse tree
	 */
		fn visit_enumType(&mut self, ctx: &EnumTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#enumDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_enumDecl(&mut self, ctx: &EnumDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#variableDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#whereExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_whereExpr(&mut self, ctx: &WhereExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#variableInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#assertExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_assertExpr(&mut self, ctx: &AssertExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#assertMainExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_assertMainExpr(&mut self, ctx: &AssertMainExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalImpliesExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalImpliesExpression(&mut self, ctx: &ConditionalImpliesExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalOrExpression(&mut self, ctx: &ConditionalOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalAndExpression(&mut self, ctx: &ConditionalAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#conditionalXorExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalXorExpression(&mut self, ctx: &ConditionalXorExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitwiseOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitwiseAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#bitShiftExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_bitShiftExpression(&mut self, ctx: &BitShiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#powExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#unaryExpressionNotPlusMinus}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpressionNotPlusMinus(&mut self, ctx: &UnaryExpressionNotPlusMinusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#oneExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_oneExpr(&mut self, ctx: &OneExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#freshExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_freshExpr(&mut self, ctx: &FreshExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#initialExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_initialExpr(&mut self, ctx: &InitialExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#prevExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_prevExpr(&mut self, ctx: &PrevExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_functionDeclaration(&mut self, ctx: &FunctionDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionBodyScope}.
	 * @param ctx the parse tree
	 */
		fn visit_functionBodyScope(&mut self, ctx: &FunctionBodyScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionParamsDecl}.
	 * @param ctx the parse tree
	 */
		fn visit_functionParamsDecl(&mut self, ctx: &FunctionParamsDeclContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#functionParam}.
	 * @param ctx the parse tree
	 */
		fn visit_functionParam(&mut self, ctx: &FunctionParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#returnExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_returnExpr(&mut self, ctx: &ReturnExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#primary}.
	 * @param ctx the parse tree
	 */
		fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#dotIdentifierExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_dotIdentifierExpr(&mut self, ctx: &DotIdentifierExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#parExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#funCall}.
	 * @param ctx the parse tree
	 */
		fn visit_funCall(&mut self, ctx: &FunCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#iteStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_iteStatement(&mut self, ctx: &IteStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CycloneParser#annotationExpr}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationExpr(&mut self, ctx: &AnnotationExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> CycloneParserVisitor<'input> for T
where
	T: CycloneParserVisitorCompat<'input>
{
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compOptions(&mut self, ctx: &CompOptionsContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_compOptions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optionName(&mut self, ctx: &OptionNameContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_optionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statementList(&mut self, ctx: &StatementListContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_statementList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transList(&mut self, ctx: &TransListContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_transList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_letOrPathAssignExpr(&mut self, ctx: &LetOrPathAssignExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_letOrPathAssignExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_globalDefinitions(&mut self, ctx: &GlobalDefinitionsContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_globalDefinitions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_machineDecl(&mut self, ctx: &MachineDeclContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_machineDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_machineScope(&mut self, ctx: &MachineScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_machineScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateExpr(&mut self, ctx: &StateExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stateExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateScope(&mut self, ctx: &StateScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stateScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trans(&mut self, ctx: &TransContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_trans(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transScope(&mut self, ctx: &TransScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_transScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transOp(&mut self, ctx: &TransOpContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_transOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transDef(&mut self, ctx: &TransDefContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_transDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transExclExpr(&mut self, ctx: &TransExclExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_transExclExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_invariantExpression(&mut self, ctx: &InvariantExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_invariantExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inExpr(&mut self, ctx: &InExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_inExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_invariantScope(&mut self, ctx: &InvariantScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_invariantScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_goal(&mut self, ctx: &GoalContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_goal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_checkExpr(&mut self, ctx: &CheckExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_checkExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forExpr(&mut self, ctx: &ForExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_forExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stopExpr(&mut self, ctx: &StopExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stopExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_viaExpr(&mut self, ctx: &ViaExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_viaExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathExprList(&mut self, ctx: &PathExprListContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathExprList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_withExpr(&mut self, ctx: &WithExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_withExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_letExpr(&mut self, ctx: &LetExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_letExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathAssignStatement(&mut self, ctx: &PathAssignStatementContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathAssignStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathCondAssignExpr(&mut self, ctx: &PathCondAssignExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathCondAssignExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathExpr(&mut self, ctx: &PathExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathCondition(&mut self, ctx: &PathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_orPathCondition(&mut self, ctx: &OrPathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_orPathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_andPathCondition(&mut self, ctx: &AndPathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_andPathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_xorPathCondition(&mut self, ctx: &XorPathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_xorPathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryPathCondition(&mut self, ctx: &UnaryPathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_unaryPathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryCondition(&mut self, ctx: &PrimaryConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_primaryCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parPathCondition(&mut self, ctx: &ParPathConditionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_parPathCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateIncExpr(&mut self, ctx: &StateIncExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stateIncExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathPrimaryExpr(&mut self, ctx: &PathPrimaryExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathPrimaryExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pathOp(&mut self, ctx: &PathOpContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_pathOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_label(&mut self, ctx: &LabelContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_label(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stateModifier(&mut self, ctx: &StateModifierContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stateModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intLiteral(&mut self, ctx: &IntLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_intLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_realLiteral(&mut self, ctx: &RealLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_realLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boolLiteral(&mut self, ctx: &BoolLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_boolLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_stringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_charLiteral(&mut self, ctx: &CharLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_charLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bvLiteral(&mut self, ctx: &BvLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_bvLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumLiteral(&mut self, ctx: &EnumLiteralContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_enumLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_record(&mut self, ctx: &RecordContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_record(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordScope(&mut self, ctx: &RecordScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_recordScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordVariableDeclGroup(&mut self, ctx: &RecordVariableDeclGroupContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_recordVariableDeclGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordVariableDecl(&mut self, ctx: &RecordVariableDeclContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_recordVariableDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_globalConstantGroup(&mut self, ctx: &GlobalConstantGroupContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_globalConstantGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_globalConstantDecl(&mut self, ctx: &GlobalConstantDeclContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_globalConstantDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_globalVariableGroup(&mut self, ctx: &GlobalVariableGroupContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_globalVariableGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localVariableGroup(&mut self, ctx: &LocalVariableGroupContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_localVariableGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_modifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_mark(&mut self, ctx: &Type_markContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_type_mark(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveBvType(&mut self, ctx: &PrimitiveBvTypeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_primitiveBvType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bvType(&mut self, ctx: &BvTypeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_bvType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_primitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumType(&mut self, ctx: &EnumTypeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_enumType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumDecl(&mut self, ctx: &EnumDeclContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_enumDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_variableDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_whereExpr(&mut self, ctx: &WhereExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_whereExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_variableInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assertExpr(&mut self, ctx: &AssertExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_assertExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assertMainExpr(&mut self, ctx: &AssertMainExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_assertMainExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalImpliesExpression(&mut self, ctx: &ConditionalImpliesExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_conditionalImpliesExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalOrExpression(&mut self, ctx: &ConditionalOrExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_conditionalOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalAndExpression(&mut self, ctx: &ConditionalAndExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_conditionalAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalXorExpression(&mut self, ctx: &ConditionalXorExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_conditionalXorExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseOrExpression(&mut self, ctx: &BitwiseOrExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_bitwiseOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwiseAndExpression(&mut self, ctx: &BitwiseAndExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_bitwiseAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_equalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_relationalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitShiftExpression(&mut self, ctx: &BitShiftExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_bitShiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_powExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpressionNotPlusMinus(&mut self, ctx: &UnaryExpressionNotPlusMinusContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_unaryExpressionNotPlusMinus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_oneExpr(&mut self, ctx: &OneExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_oneExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_freshExpr(&mut self, ctx: &FreshExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_freshExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initialExpr(&mut self, ctx: &InitialExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_initialExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prevExpr(&mut self, ctx: &PrevExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_prevExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionDeclaration(&mut self, ctx: &FunctionDeclarationContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_functionDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionBodyScope(&mut self, ctx: &FunctionBodyScopeContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_functionBodyScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionParamsDecl(&mut self, ctx: &FunctionParamsDeclContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_functionParamsDecl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionParam(&mut self, ctx: &FunctionParamContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_functionParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_returnExpr(&mut self, ctx: &ReturnExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_returnExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dotIdentifierExpr(&mut self, ctx: &DotIdentifierExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_dotIdentifierExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_parExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_funCall(&mut self, ctx: &FunCallContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_funCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iteStatement(&mut self, ctx: &IteStatementContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_iteStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationExpr(&mut self, ctx: &AnnotationExprContext<'input>){
		let result = <Self as CycloneParserVisitorCompat>::visit_annotationExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}