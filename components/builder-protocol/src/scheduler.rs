// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::result;
use message::Routable;

pub use message::scheduler::*;
use message::depotsrv;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

impl From<depotsrv::Package> for Package {
    fn from(value: depotsrv::Package) -> Package {
        let mut package = Package::new();

        let name = format!("{}", value.get_ident());

        let deps = value
            .get_deps()
            .iter()
            .map(|x| format!("{}", x))
            .collect();

        package.set_ident(name);
        package.set_deps(deps);
        package
    }
}

impl Routable for GroupCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(format!("{}/{}", self.get_origin(), self.get_package()))
    }
}

impl Routable for GroupGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_group_id().to_string())
    }
}

impl Serialize for GroupState {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self as u64 {
            0 => serializer.serialize_str("Pending"),
            1 => serializer.serialize_str("Dispatching"),
            2 => serializer.serialize_str("Complete"),
            3 => serializer.serialize_str("Failed"),
            _ => panic!("Unexpected enum value"),
        }
    }
}

impl Serialize for ProjectState {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self as u64 {
            0 => serializer.serialize_str("NotStarted"),
            1 => serializer.serialize_str("InProgress"),
            2 => serializer.serialize_str("Success"),
            3 => serializer.serialize_str("Failure"),
            _ => panic!("Unexpected enum value"),
        }
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("project", 3));
        try!(strukt.serialize_field("name", &self.get_name()));
        try!(strukt.serialize_field("state", &self.get_state()));
        try!(strukt.serialize_field("job_id", &self.get_job_id()));
        strukt.end()
    }
}

impl Serialize for Group {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("group", 3));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("state", &self.get_state()));
        try!(strukt.serialize_field("projects", &self.get_projects()));
        strukt.end()
    }
}
