// module responsible for connecting to
// rocksdb
pub mod rdb_connection {
    use serde::{Deserialize, Serialize};
    use utils::serializer;

    // enum with with Db or Nil object
    // TODO: see if this can be handled with
    // Option or Result
    pub enum DB {
        Db(rocksdb::DB),
        Nil,
    }
    // struct with actual connection object and
    // boolean to check if connected
    // TODO: see if connected is really needed
    pub struct Con {
        con_obj: DB,
        connected: bool,
    }

    impl Con {
        // initialize a new object to make connection
        pub fn new() -> Con {
            let obj = Con {
                con_obj: DB::Nil,
                connected: false,
            };
            obj
        }
        pub fn connect(&mut self) {
            // Todo :: handle if not able to connect
            // Take input name of DB(but review the
            //security pov with that functionality )
            self.con_obj = DB::Db(rocksdb::DB::open_default("rockdb/db").unwrap());
            self.connected = true;
        }

        // put a serializable object with its serialized value in
        // DB with key = hash of serialized value
        // returns None if Not connected to db
        pub fn put_in_db<T>(&self, object: T) -> Option<Vec<u8>>
        where
            T: Serialize,
        {
            match &self.con_obj {
                DB::Db(some) => {
                    let ser_obj = serializer::serialize(&object);
                    let hash_obj = serializer::serialize_hash256(&object);
                    let _res = some.put(&hash_obj, ser_obj);
                    Some(hash_obj)
                }
                DB::Nil => {
                    // println!("Not connected to db");
                    None
                }
            }
        }
        // input key as vec<u8> retrieves serialized value
        // of object from db , returns empty vector if
        // not connected to db
        pub fn getu8(self, slice: &[u8]) -> Vec<u8> {
            match self.con_obj {
                DB::Db(some) => {
                    //TODO: handle possible errors here
                    //some.get(slice) gives Result
                    // which can be Ok(Some(value)), Ok(None)
                    // or Err(e)
                    some.get(slice).unwrap().unwrap()
                }
                DB::Nil => vec![],
            }
        }
        // NOTE: WILL NOT WORK AS OF NOW
        // Expected functionality: take input key
        // retrive the corresponding value from db
        // use that value to get the deserialized object
        // if any error return None

        // ISSUES: Not able to get deserialized object using srde_cbor
        //     most likely issue with lifetime of the value
        //     (cannot be of lifetime that the caller function is to which the
        //     the object will be returned and eventually bound to a type
        pub fn get_from_db<'a, T>(self, slice: &'a [u8]) -> Option<T>
        where
            T: Deserialize<'a>,
        {
            match self.con_obj {
                DB::Db(some) => {
                    let x = some.get(&slice);
                    //TODO: rather than is_ok handle
                    // all possibilities using match
                    if x.is_ok() {
                        let v = &x.unwrap().unwrap();
                        // println!("slice is {:?}",v);
                        serde_cbor::from_slice(&v).unwrap()
                    }
                    None

                    // Following snippet can be used to
                    // handle all cases properly and
                    // prevent the thread from panicking later
                    // let res = some.get(slice);
                    // match res{

                    //     Ok(Some(value)) =>{
                    //         serde_cbor::from_slice(&value).unwrap()
                    //         // let a = res.unwrap().unwrap();
                    //         sbserde::sb_deser(res.unwrap()_)
                    //     },
                    //     Ok(None) => None,
                    //     Err(_e) => {
                    //         None
                    //     }
                    // }
                }
                DB::Nil => None,
            }
            // Ok(value)
        }
    }
}
