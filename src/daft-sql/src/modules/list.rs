use daft_core::prelude::CountMode;
use daft_dsl::{lit, Expr, LiteralValue};

use super::SQLModule;
use crate::{
    error::PlannerError,
    functions::{SQLFunction, SQLFunctions},
    unsupported_sql_err,
};

pub struct SQLModuleList;

impl SQLModule for SQLModuleList {
    fn register(parent: &mut SQLFunctions) {
        parent.add_fn("list_chunk", SQLListChunk);
        parent.add_fn("list_count", SQLListCount);
        parent.add_fn("explode", SQLExplode);
        parent.add_fn("unnest", SQLExplode);
        // this is commonly called `array_to_string` in other SQL dialects
        parent.add_fn("array_to_string", SQLListJoin);
        // but we also want to support our `list_join` alias as well
        parent.add_fn("list_join", SQLListJoin);
        parent.add_fn("list_max", SQLListMax);
        parent.add_fn("list_min", SQLListMin);
        parent.add_fn("list_sum", SQLListSum);
        parent.add_fn("list_mean", SQLListMean);
        parent.add_fn("list_slice", SQLListSlice);
        parent.add_fn("list_sort", SQLListSort);

        // TODO
    }
}

pub struct SQLListChunk;

impl SQLFunction for SQLListChunk {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input, chunk_size] => {
                let input = planner.plan_function_arg(input)?;
                let chunk_size = planner
                    .plan_function_arg(chunk_size)
                    .and_then(|arg| match arg.as_ref() {
                        Expr::Literal(LiteralValue::Int64(n)) => Ok(*n as usize),
                        _ => unsupported_sql_err!("Expected chunk size to be a number"),
                    })?;
                Ok(daft_functions::list::chunk(input, chunk_size))
            }
            _ => unsupported_sql_err!(
                "invalid arguments for list_chunk. Expected list_chunk(expr, chunk_size)"
            ),
        }
    }
}

pub struct SQLListCount;

impl SQLFunction for SQLListCount {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::count(input, CountMode::Valid))
            }
            [input, count_mode] => {
                let input = planner.plan_function_arg(input)?;
                let mode =
                    planner
                        .plan_function_arg(count_mode)
                        .and_then(|arg| match arg.as_ref() {
                            Expr::Literal(LiteralValue::Utf8(s)) => {
                                s.parse().map_err(PlannerError::from)
                            }
                            _ => unsupported_sql_err!("Expected mode to be a string"),
                        })?;
                Ok(daft_functions::list::count(input, mode))
            }
            _ => unsupported_sql_err!("invalid arguments for list_count. Expected either list_count(expr) or list_count(expr, mode)"),
        }
    }
}

pub struct SQLExplode;

impl SQLFunction for SQLExplode {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::explode(input))
            }
            _ => unsupported_sql_err!("Expected 1 argument"),
        }
    }
}

pub struct SQLListJoin;

impl SQLFunction for SQLListJoin {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input, separator] => {
                let input = planner.plan_function_arg(input)?;
                let separator = planner.plan_function_arg(separator)?;
                Ok(daft_functions::list::join(input, separator))
            }
            _ => unsupported_sql_err!(
                "invalid arguments for list_join. Expected list_join(expr, separator)"
            ),
        }
    }
}

pub struct SQLListMax;

impl SQLFunction for SQLListMax {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::max(input))
            }
            _ => unsupported_sql_err!("invalid arguments for list_max. Expected list_max(expr)"),
        }
    }
}

pub struct SQLListMean;

impl SQLFunction for SQLListMean {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::mean(input))
            }
            _ => unsupported_sql_err!("invalid arguments for list_mean. Expected list_mean(expr)"),
        }
    }
}

pub struct SQLListMin;

impl SQLFunction for SQLListMin {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::min(input))
            }
            _ => unsupported_sql_err!("invalid arguments for list_min. Expected list_min(expr)"),
        }
    }
}

pub struct SQLListSum;

impl SQLFunction for SQLListSum {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::sum(input))
            }
            _ => unsupported_sql_err!("invalid arguments for list_sum. Expected list_sum(expr)"),
        }
    }
}

pub struct SQLListSlice;

impl SQLFunction for SQLListSlice {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input, start, end] => {
                let input = planner.plan_function_arg(input)?;
                let start = planner.plan_function_arg(start)?;
                let end = planner.plan_function_arg(end)?;
                Ok(daft_functions::list::slice(input, start, end))
            }
            _ => unsupported_sql_err!(
                "invalid arguments for list_slice. Expected list_slice(expr, start, end)"
            ),
        }
    }
}

pub struct SQLListSort;

impl SQLFunction for SQLListSort {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> crate::error::SQLPlannerResult<daft_dsl::ExprRef> {
        match inputs {
            [input] => {
                let input = planner.plan_function_arg(input)?;
                Ok(daft_functions::list::sort(input, None))
            }
            [input, order] => {
                let input = planner.plan_function_arg(input)?;
                use sqlparser::ast::{
                    Expr::Identifier as SQLIdent, FunctionArg::Unnamed,
                    FunctionArgExpr::Expr as SQLExpr,
                };

                let order = match order {
                    Unnamed(SQLExpr(SQLIdent(ident))) => {
                        match ident.value.to_lowercase().as_str() {
                            "asc" => lit(false),
                            "desc" => lit(true),
                            _ => unsupported_sql_err!("invalid order for list_sort"),
                        }
                    }
                    _ => unsupported_sql_err!("invalid order for list_sort"),
                };
                Ok(daft_functions::list::sort(input, Some(order)))
            }
            _ => unsupported_sql_err!(
                "invalid arguments for list_sort. Expected list_sort(expr, ASC|DESC)"
            ),
        }
    }
}