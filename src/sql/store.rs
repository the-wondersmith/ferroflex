// // Implementation of GlueSQL's required `Store` and `StoreMut` traits for DataFlex table files
//
// // Third-Party Imports
// use async_trait::async_trait;
// use gluesql::core::data::{Schema};
// use gluesql::core::result::Result as SqlResult;
// // use gluesql::core::result::MutResult as MutSqlResult;
// use gluesql::core::store::{RowIter, Store}; // StoreMut
//
//
// // Crate-Level Imports
// use crate::structs::database::DataFlexDB;
// // use crate::structs::table::{DataFlexTable, Row};
//
// #[allow(unused_variables)]
// #[async_trait(?Send)]
// impl Store<()> for DataFlexDB {
//     async fn fetch_schema(&self, table_name: &str) -> SqlResult<Option<Schema>> {
//         todo!()
//     }
//
//     async fn scan_data(&self, table_name: &str) -> SqlResult<RowIter<()>> {
//         todo!()
//     }
// }
//
// #[allow(unused_variables)]
// #[async_trait]
// impl StoreMut<Row> for DataFlexDB {
//     async fn insert_schema(self, schema: &Schema) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
//
//     async fn delete_schema(self, table_name: &str) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
//
//     async fn insert_data(self, table_name: &str, rows: Vec<SqlRow>) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
//
//     async fn update_data(
//         self,
//         table_name: &str,
//         rows: Vec<(DataFlexTable, SqlRow)>,
//     ) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
//
//     async fn delete_data(self, table_name: &str, keys: Vec<u32>) -> MutSqlResult<Self, ()> {
//         todo!()
//     }
// }
