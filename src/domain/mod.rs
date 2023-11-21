//! Contains the business logic and domain models for the application
//! Each sub module centers around a specific entity or a group of entities.
//! Each entity has a central struct which represents the data as it is
//! stored in the database. Further, it has a collection of DTOs, which
//! can be considered views over the data and can be used for various purposes.
//!
//! The database model is typically for internal use. It should usually be
//! converted to a DTO be returning as a response.

pub mod user;
