// // Implementation of GlueSQL's optional `Metadata` trait for DataFlex table files
//
// // Third-Party Imports
// use async_trait::async_trait;
// use gluesql::core::result::Result as SqlResult;
// use gluesql::core::store::Metadata;
//
// // Crate-Level Imports
// use crate::structs::database::DataFlexDB;
//
// #[async_trait]
// impl Metadata for DataFlexDB {
//     async fn version(&self) -> String {
//         todo!()
//     }
//
//     async fn schema_names(&self) -> SqlResult<Vec<String>> {
//         todo!()
//     }
// }
