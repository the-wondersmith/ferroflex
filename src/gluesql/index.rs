// // Implementation of GlueSQL's optional `Index` and `IndexMut` traits for DataFlex table files
//
// // Standard Library Imports
// use std::fmt::Debug;
//
// // Third-Party Imports
// use async_trait::async_trait;
// use gluesql::core::ast::{IndexOperator, OrderByExpr};
// use gluesql::core::result::{MutResult as MutSqlResult, Result as SqlResult};
// use gluesql::core::store::{Index as SqlIndex, IndexMut as SqlIndexMut, RowIter};
// use gluesql::prelude::*;
//
// // Crate-Level Imports
// use crate::structs::index::Index;
//
// #[allow(unused_variables)]
// #[async_trait]
// impl<T: Debug> SqlIndex<T> for Index {
//     async fn scan_indexed_data(
//         &self,
//         table_name: &str,
//         index_name: &str,
//         asc: Option<bool>,
//         cmp_value: Option<(&IndexOperator, Value)>,
//     ) -> SqlResult<RowIter<T>> {
//         todo!()
//     }
// }
//
// #[allow(unused_variables)]
// #[async_trait]
// impl<T: Debug> SqlIndexMut<T> for Index {
//     async fn create_index(
//         self,
//         table_name: &str,
//         index_name: &str,
//         column: &OrderByExpr,
//     ) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
//
//     async fn drop_index(self, table_name: &str, index_name: &str) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
// }
