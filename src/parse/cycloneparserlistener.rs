#![allow(nonstandard_style)]
// Generated from grammar/CycloneParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::cycloneparser::*;

pub trait CycloneParserListener<'input> : ParseTreeListener<'input,CycloneParserContextType>{
/**
 * Enter a parse tree produced by {@link CycloneParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#compOptions}.
 * @param ctx the parse tree
 */
fn enter_compOptions(&mut self, _ctx: &CompOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#compOptions}.
 * @param ctx the parse tree
 */
fn exit_compOptions(&mut self, _ctx: &CompOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#optionName}.
 * @param ctx the parse tree
 */
fn enter_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#optionName}.
 * @param ctx the parse tree
 */
fn exit_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#statementList}.
 * @param ctx the parse tree
 */
fn enter_statementList(&mut self, _ctx: &StatementListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#statementList}.
 * @param ctx the parse tree
 */
fn exit_statementList(&mut self, _ctx: &StatementListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#transList}.
 * @param ctx the parse tree
 */
fn enter_transList(&mut self, _ctx: &TransListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#transList}.
 * @param ctx the parse tree
 */
fn exit_transList(&mut self, _ctx: &TransListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#letOrPathAssignExpr}.
 * @param ctx the parse tree
 */
fn enter_letOrPathAssignExpr(&mut self, _ctx: &LetOrPathAssignExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#letOrPathAssignExpr}.
 * @param ctx the parse tree
 */
fn exit_letOrPathAssignExpr(&mut self, _ctx: &LetOrPathAssignExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#globalDefinitions}.
 * @param ctx the parse tree
 */
fn enter_globalDefinitions(&mut self, _ctx: &GlobalDefinitionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#globalDefinitions}.
 * @param ctx the parse tree
 */
fn exit_globalDefinitions(&mut self, _ctx: &GlobalDefinitionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#machineDecl}.
 * @param ctx the parse tree
 */
fn enter_machineDecl(&mut self, _ctx: &MachineDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#machineDecl}.
 * @param ctx the parse tree
 */
fn exit_machineDecl(&mut self, _ctx: &MachineDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#machineScope}.
 * @param ctx the parse tree
 */
fn enter_machineScope(&mut self, _ctx: &MachineScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#machineScope}.
 * @param ctx the parse tree
 */
fn exit_machineScope(&mut self, _ctx: &MachineScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stateExpr}.
 * @param ctx the parse tree
 */
fn enter_stateExpr(&mut self, _ctx: &StateExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stateExpr}.
 * @param ctx the parse tree
 */
fn exit_stateExpr(&mut self, _ctx: &StateExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stateScope}.
 * @param ctx the parse tree
 */
fn enter_stateScope(&mut self, _ctx: &StateScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stateScope}.
 * @param ctx the parse tree
 */
fn exit_stateScope(&mut self, _ctx: &StateScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#trans}.
 * @param ctx the parse tree
 */
fn enter_trans(&mut self, _ctx: &TransContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#trans}.
 * @param ctx the parse tree
 */
fn exit_trans(&mut self, _ctx: &TransContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#transScope}.
 * @param ctx the parse tree
 */
fn enter_transScope(&mut self, _ctx: &TransScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#transScope}.
 * @param ctx the parse tree
 */
fn exit_transScope(&mut self, _ctx: &TransScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#transOp}.
 * @param ctx the parse tree
 */
fn enter_transOp(&mut self, _ctx: &TransOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#transOp}.
 * @param ctx the parse tree
 */
fn exit_transOp(&mut self, _ctx: &TransOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#transDef}.
 * @param ctx the parse tree
 */
fn enter_transDef(&mut self, _ctx: &TransDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#transDef}.
 * @param ctx the parse tree
 */
fn exit_transDef(&mut self, _ctx: &TransDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#transExclExpr}.
 * @param ctx the parse tree
 */
fn enter_transExclExpr(&mut self, _ctx: &TransExclExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#transExclExpr}.
 * @param ctx the parse tree
 */
fn exit_transExclExpr(&mut self, _ctx: &TransExclExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#invariantExpression}.
 * @param ctx the parse tree
 */
fn enter_invariantExpression(&mut self, _ctx: &InvariantExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#invariantExpression}.
 * @param ctx the parse tree
 */
fn exit_invariantExpression(&mut self, _ctx: &InvariantExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#inExpr}.
 * @param ctx the parse tree
 */
fn enter_inExpr(&mut self, _ctx: &InExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#inExpr}.
 * @param ctx the parse tree
 */
fn exit_inExpr(&mut self, _ctx: &InExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#invariantScope}.
 * @param ctx the parse tree
 */
fn enter_invariantScope(&mut self, _ctx: &InvariantScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#invariantScope}.
 * @param ctx the parse tree
 */
fn exit_invariantScope(&mut self, _ctx: &InvariantScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#goal}.
 * @param ctx the parse tree
 */
fn enter_goal(&mut self, _ctx: &GoalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#goal}.
 * @param ctx the parse tree
 */
fn exit_goal(&mut self, _ctx: &GoalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#checkExpr}.
 * @param ctx the parse tree
 */
fn enter_checkExpr(&mut self, _ctx: &CheckExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#checkExpr}.
 * @param ctx the parse tree
 */
fn exit_checkExpr(&mut self, _ctx: &CheckExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#forExpr}.
 * @param ctx the parse tree
 */
fn enter_forExpr(&mut self, _ctx: &ForExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#forExpr}.
 * @param ctx the parse tree
 */
fn exit_forExpr(&mut self, _ctx: &ForExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stopExpr}.
 * @param ctx the parse tree
 */
fn enter_stopExpr(&mut self, _ctx: &StopExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stopExpr}.
 * @param ctx the parse tree
 */
fn exit_stopExpr(&mut self, _ctx: &StopExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#viaExpr}.
 * @param ctx the parse tree
 */
fn enter_viaExpr(&mut self, _ctx: &ViaExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#viaExpr}.
 * @param ctx the parse tree
 */
fn exit_viaExpr(&mut self, _ctx: &ViaExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathExprList}.
 * @param ctx the parse tree
 */
fn enter_pathExprList(&mut self, _ctx: &PathExprListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathExprList}.
 * @param ctx the parse tree
 */
fn exit_pathExprList(&mut self, _ctx: &PathExprListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#withExpr}.
 * @param ctx the parse tree
 */
fn enter_withExpr(&mut self, _ctx: &WithExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#withExpr}.
 * @param ctx the parse tree
 */
fn exit_withExpr(&mut self, _ctx: &WithExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#letExpr}.
 * @param ctx the parse tree
 */
fn enter_letExpr(&mut self, _ctx: &LetExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#letExpr}.
 * @param ctx the parse tree
 */
fn exit_letExpr(&mut self, _ctx: &LetExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathAssignStatement}.
 * @param ctx the parse tree
 */
fn enter_pathAssignStatement(&mut self, _ctx: &PathAssignStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathAssignStatement}.
 * @param ctx the parse tree
 */
fn exit_pathAssignStatement(&mut self, _ctx: &PathAssignStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathCondAssignExpr}.
 * @param ctx the parse tree
 */
fn enter_pathCondAssignExpr(&mut self, _ctx: &PathCondAssignExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathCondAssignExpr}.
 * @param ctx the parse tree
 */
fn exit_pathCondAssignExpr(&mut self, _ctx: &PathCondAssignExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathExpr}.
 * @param ctx the parse tree
 */
fn enter_pathExpr(&mut self, _ctx: &PathExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathExpr}.
 * @param ctx the parse tree
 */
fn exit_pathExpr(&mut self, _ctx: &PathExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathCondition}.
 * @param ctx the parse tree
 */
fn enter_pathCondition(&mut self, _ctx: &PathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathCondition}.
 * @param ctx the parse tree
 */
fn exit_pathCondition(&mut self, _ctx: &PathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#orPathCondition}.
 * @param ctx the parse tree
 */
fn enter_orPathCondition(&mut self, _ctx: &OrPathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#orPathCondition}.
 * @param ctx the parse tree
 */
fn exit_orPathCondition(&mut self, _ctx: &OrPathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#andPathCondition}.
 * @param ctx the parse tree
 */
fn enter_andPathCondition(&mut self, _ctx: &AndPathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#andPathCondition}.
 * @param ctx the parse tree
 */
fn exit_andPathCondition(&mut self, _ctx: &AndPathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#xorPathCondition}.
 * @param ctx the parse tree
 */
fn enter_xorPathCondition(&mut self, _ctx: &XorPathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#xorPathCondition}.
 * @param ctx the parse tree
 */
fn exit_xorPathCondition(&mut self, _ctx: &XorPathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#unaryPathCondition}.
 * @param ctx the parse tree
 */
fn enter_unaryPathCondition(&mut self, _ctx: &UnaryPathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#unaryPathCondition}.
 * @param ctx the parse tree
 */
fn exit_unaryPathCondition(&mut self, _ctx: &UnaryPathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#primaryCondition}.
 * @param ctx the parse tree
 */
fn enter_primaryCondition(&mut self, _ctx: &PrimaryConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#primaryCondition}.
 * @param ctx the parse tree
 */
fn exit_primaryCondition(&mut self, _ctx: &PrimaryConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#parPathCondition}.
 * @param ctx the parse tree
 */
fn enter_parPathCondition(&mut self, _ctx: &ParPathConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#parPathCondition}.
 * @param ctx the parse tree
 */
fn exit_parPathCondition(&mut self, _ctx: &ParPathConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stateIncExpr}.
 * @param ctx the parse tree
 */
fn enter_stateIncExpr(&mut self, _ctx: &StateIncExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stateIncExpr}.
 * @param ctx the parse tree
 */
fn exit_stateIncExpr(&mut self, _ctx: &StateIncExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathPrimaryExpr}.
 * @param ctx the parse tree
 */
fn enter_pathPrimaryExpr(&mut self, _ctx: &PathPrimaryExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathPrimaryExpr}.
 * @param ctx the parse tree
 */
fn exit_pathPrimaryExpr(&mut self, _ctx: &PathPrimaryExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#pathOp}.
 * @param ctx the parse tree
 */
fn enter_pathOp(&mut self, _ctx: &PathOpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#pathOp}.
 * @param ctx the parse tree
 */
fn exit_pathOp(&mut self, _ctx: &PathOpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#label}.
 * @param ctx the parse tree
 */
fn enter_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#label}.
 * @param ctx the parse tree
 */
fn exit_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stateModifier}.
 * @param ctx the parse tree
 */
fn enter_stateModifier(&mut self, _ctx: &StateModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stateModifier}.
 * @param ctx the parse tree
 */
fn exit_stateModifier(&mut self, _ctx: &StateModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#intLiteral}.
 * @param ctx the parse tree
 */
fn enter_intLiteral(&mut self, _ctx: &IntLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#intLiteral}.
 * @param ctx the parse tree
 */
fn exit_intLiteral(&mut self, _ctx: &IntLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#realLiteral}.
 * @param ctx the parse tree
 */
fn enter_realLiteral(&mut self, _ctx: &RealLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#realLiteral}.
 * @param ctx the parse tree
 */
fn exit_realLiteral(&mut self, _ctx: &RealLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#boolLiteral}.
 * @param ctx the parse tree
 */
fn enter_boolLiteral(&mut self, _ctx: &BoolLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#boolLiteral}.
 * @param ctx the parse tree
 */
fn exit_boolLiteral(&mut self, _ctx: &BoolLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#charLiteral}.
 * @param ctx the parse tree
 */
fn enter_charLiteral(&mut self, _ctx: &CharLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#charLiteral}.
 * @param ctx the parse tree
 */
fn exit_charLiteral(&mut self, _ctx: &CharLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#bvLiteral}.
 * @param ctx the parse tree
 */
fn enter_bvLiteral(&mut self, _ctx: &BvLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#bvLiteral}.
 * @param ctx the parse tree
 */
fn exit_bvLiteral(&mut self, _ctx: &BvLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#enumLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumLiteral(&mut self, _ctx: &EnumLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#enumLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumLiteral(&mut self, _ctx: &EnumLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#record}.
 * @param ctx the parse tree
 */
fn enter_record(&mut self, _ctx: &RecordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#record}.
 * @param ctx the parse tree
 */
fn exit_record(&mut self, _ctx: &RecordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#recordScope}.
 * @param ctx the parse tree
 */
fn enter_recordScope(&mut self, _ctx: &RecordScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#recordScope}.
 * @param ctx the parse tree
 */
fn exit_recordScope(&mut self, _ctx: &RecordScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#recordVariableDeclGroup}.
 * @param ctx the parse tree
 */
fn enter_recordVariableDeclGroup(&mut self, _ctx: &RecordVariableDeclGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#recordVariableDeclGroup}.
 * @param ctx the parse tree
 */
fn exit_recordVariableDeclGroup(&mut self, _ctx: &RecordVariableDeclGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#recordVariableDecl}.
 * @param ctx the parse tree
 */
fn enter_recordVariableDecl(&mut self, _ctx: &RecordVariableDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#recordVariableDecl}.
 * @param ctx the parse tree
 */
fn exit_recordVariableDecl(&mut self, _ctx: &RecordVariableDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#globalConstantGroup}.
 * @param ctx the parse tree
 */
fn enter_globalConstantGroup(&mut self, _ctx: &GlobalConstantGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#globalConstantGroup}.
 * @param ctx the parse tree
 */
fn exit_globalConstantGroup(&mut self, _ctx: &GlobalConstantGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#globalConstantDecl}.
 * @param ctx the parse tree
 */
fn enter_globalConstantDecl(&mut self, _ctx: &GlobalConstantDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#globalConstantDecl}.
 * @param ctx the parse tree
 */
fn exit_globalConstantDecl(&mut self, _ctx: &GlobalConstantDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#globalVariableGroup}.
 * @param ctx the parse tree
 */
fn enter_globalVariableGroup(&mut self, _ctx: &GlobalVariableGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#globalVariableGroup}.
 * @param ctx the parse tree
 */
fn exit_globalVariableGroup(&mut self, _ctx: &GlobalVariableGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#localVariableGroup}.
 * @param ctx the parse tree
 */
fn enter_localVariableGroup(&mut self, _ctx: &LocalVariableGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#localVariableGroup}.
 * @param ctx the parse tree
 */
fn exit_localVariableGroup(&mut self, _ctx: &LocalVariableGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#modifier}.
 * @param ctx the parse tree
 */
fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#modifier}.
 * @param ctx the parse tree
 */
fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#type_mark}.
 * @param ctx the parse tree
 */
fn enter_type_mark(&mut self, _ctx: &Type_markContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#type_mark}.
 * @param ctx the parse tree
 */
fn exit_type_mark(&mut self, _ctx: &Type_markContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#primitiveBvType}.
 * @param ctx the parse tree
 */
fn enter_primitiveBvType(&mut self, _ctx: &PrimitiveBvTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#primitiveBvType}.
 * @param ctx the parse tree
 */
fn exit_primitiveBvType(&mut self, _ctx: &PrimitiveBvTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#bvType}.
 * @param ctx the parse tree
 */
fn enter_bvType(&mut self, _ctx: &BvTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#bvType}.
 * @param ctx the parse tree
 */
fn exit_bvType(&mut self, _ctx: &BvTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#enumType}.
 * @param ctx the parse tree
 */
fn enter_enumType(&mut self, _ctx: &EnumTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#enumType}.
 * @param ctx the parse tree
 */
fn exit_enumType(&mut self, _ctx: &EnumTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#enumDecl}.
 * @param ctx the parse tree
 */
fn enter_enumDecl(&mut self, _ctx: &EnumDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#enumDecl}.
 * @param ctx the parse tree
 */
fn exit_enumDecl(&mut self, _ctx: &EnumDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#whereExpr}.
 * @param ctx the parse tree
 */
fn enter_whereExpr(&mut self, _ctx: &WhereExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#whereExpr}.
 * @param ctx the parse tree
 */
fn exit_whereExpr(&mut self, _ctx: &WhereExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#variableInitializer}.
 * @param ctx the parse tree
 */
fn enter_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#variableInitializer}.
 * @param ctx the parse tree
 */
fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#assertExpr}.
 * @param ctx the parse tree
 */
fn enter_assertExpr(&mut self, _ctx: &AssertExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#assertExpr}.
 * @param ctx the parse tree
 */
fn exit_assertExpr(&mut self, _ctx: &AssertExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#assertMainExpr}.
 * @param ctx the parse tree
 */
fn enter_assertMainExpr(&mut self, _ctx: &AssertMainExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#assertMainExpr}.
 * @param ctx the parse tree
 */
fn exit_assertMainExpr(&mut self, _ctx: &AssertMainExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#conditionalImpliesExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalImpliesExpression(&mut self, _ctx: &ConditionalImpliesExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#conditionalImpliesExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalImpliesExpression(&mut self, _ctx: &ConditionalImpliesExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#conditionalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalOrExpression(&mut self, _ctx: &ConditionalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#conditionalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalOrExpression(&mut self, _ctx: &ConditionalOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#conditionalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalAndExpression(&mut self, _ctx: &ConditionalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#conditionalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalAndExpression(&mut self, _ctx: &ConditionalAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#conditionalXorExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalXorExpression(&mut self, _ctx: &ConditionalXorExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#conditionalXorExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalXorExpression(&mut self, _ctx: &ConditionalXorExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#bitwiseOrExpression}.
 * @param ctx the parse tree
 */
fn enter_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#bitwiseOrExpression}.
 * @param ctx the parse tree
 */
fn exit_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#bitwiseAndExpression}.
 * @param ctx the parse tree
 */
fn enter_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#bitwiseAndExpression}.
 * @param ctx the parse tree
 */
fn exit_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#relationalExpression}.
 * @param ctx the parse tree
 */
fn enter_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#relationalExpression}.
 * @param ctx the parse tree
 */
fn exit_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#bitShiftExpression}.
 * @param ctx the parse tree
 */
fn enter_bitShiftExpression(&mut self, _ctx: &BitShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#bitShiftExpression}.
 * @param ctx the parse tree
 */
fn exit_bitShiftExpression(&mut self, _ctx: &BitShiftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#powExpression}.
 * @param ctx the parse tree
 */
fn enter_powExpression(&mut self, _ctx: &PowExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#powExpression}.
 * @param ctx the parse tree
 */
fn exit_powExpression(&mut self, _ctx: &PowExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#unaryExpressionNotPlusMinus}.
 * @param ctx the parse tree
 */
fn enter_unaryExpressionNotPlusMinus(&mut self, _ctx: &UnaryExpressionNotPlusMinusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#unaryExpressionNotPlusMinus}.
 * @param ctx the parse tree
 */
fn exit_unaryExpressionNotPlusMinus(&mut self, _ctx: &UnaryExpressionNotPlusMinusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#oneExpr}.
 * @param ctx the parse tree
 */
fn enter_oneExpr(&mut self, _ctx: &OneExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#oneExpr}.
 * @param ctx the parse tree
 */
fn exit_oneExpr(&mut self, _ctx: &OneExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#freshExpr}.
 * @param ctx the parse tree
 */
fn enter_freshExpr(&mut self, _ctx: &FreshExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#freshExpr}.
 * @param ctx the parse tree
 */
fn exit_freshExpr(&mut self, _ctx: &FreshExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#initialExpr}.
 * @param ctx the parse tree
 */
fn enter_initialExpr(&mut self, _ctx: &InitialExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#initialExpr}.
 * @param ctx the parse tree
 */
fn exit_initialExpr(&mut self, _ctx: &InitialExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#prevExpr}.
 * @param ctx the parse tree
 */
fn enter_prevExpr(&mut self, _ctx: &PrevExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#prevExpr}.
 * @param ctx the parse tree
 */
fn exit_prevExpr(&mut self, _ctx: &PrevExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#functionDeclaration}.
 * @param ctx the parse tree
 */
fn enter_functionDeclaration(&mut self, _ctx: &FunctionDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#functionDeclaration}.
 * @param ctx the parse tree
 */
fn exit_functionDeclaration(&mut self, _ctx: &FunctionDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#functionBodyScope}.
 * @param ctx the parse tree
 */
fn enter_functionBodyScope(&mut self, _ctx: &FunctionBodyScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#functionBodyScope}.
 * @param ctx the parse tree
 */
fn exit_functionBodyScope(&mut self, _ctx: &FunctionBodyScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#functionParamsDecl}.
 * @param ctx the parse tree
 */
fn enter_functionParamsDecl(&mut self, _ctx: &FunctionParamsDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#functionParamsDecl}.
 * @param ctx the parse tree
 */
fn exit_functionParamsDecl(&mut self, _ctx: &FunctionParamsDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#functionParam}.
 * @param ctx the parse tree
 */
fn enter_functionParam(&mut self, _ctx: &FunctionParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#functionParam}.
 * @param ctx the parse tree
 */
fn exit_functionParam(&mut self, _ctx: &FunctionParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#returnExpr}.
 * @param ctx the parse tree
 */
fn enter_returnExpr(&mut self, _ctx: &ReturnExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#returnExpr}.
 * @param ctx the parse tree
 */
fn exit_returnExpr(&mut self, _ctx: &ReturnExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#dotIdentifierExpr}.
 * @param ctx the parse tree
 */
fn enter_dotIdentifierExpr(&mut self, _ctx: &DotIdentifierExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#dotIdentifierExpr}.
 * @param ctx the parse tree
 */
fn exit_dotIdentifierExpr(&mut self, _ctx: &DotIdentifierExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#parExpression}.
 * @param ctx the parse tree
 */
fn enter_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#parExpression}.
 * @param ctx the parse tree
 */
fn exit_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#funCall}.
 * @param ctx the parse tree
 */
fn enter_funCall(&mut self, _ctx: &FunCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#funCall}.
 * @param ctx the parse tree
 */
fn exit_funCall(&mut self, _ctx: &FunCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#iteStatement}.
 * @param ctx the parse tree
 */
fn enter_iteStatement(&mut self, _ctx: &IteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#iteStatement}.
 * @param ctx the parse tree
 */
fn exit_iteStatement(&mut self, _ctx: &IteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link CycloneParser#annotationExpr}.
 * @param ctx the parse tree
 */
fn enter_annotationExpr(&mut self, _ctx: &AnnotationExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CycloneParser#annotationExpr}.
 * @param ctx the parse tree
 */
fn exit_annotationExpr(&mut self, _ctx: &AnnotationExprContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : CycloneParserListener<'input> }


