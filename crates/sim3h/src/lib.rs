extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate dynomite;
extern crate lib3h_protocol;

use lib3h_protocol::protocol_client::Lib3hClientProtocol;


/// mimic lib3h::engine::real_engine::serve_Lib3hClientProtocol
pub fn serve_Lib3hClient(InFromCore)Protocol(client_msg: Lib3hClientProtocol) {
    debug!("serving: {:?}", client_msg);

    /// docs for all sequences at:
    /// https://hackmd.io/Rag5au4dQfm1CtcjOK7y5w
    match protocol {
        Lib3hClientProtocol::Shutdown => {
            // ** do nothing **
            // this is a hangover from n3h
        },

        // this doesn't do anything standalone
        Lib3hClientProtocol::SuccessResult(generic_result_data) => { generic_result_data; },

        // this doesn't do anything standalone
        Lib3hClientProtocol::FailureResult(generic_result_data) => { generic_result_data; },

        // https://hackmd.io/Rag5au4dQfm1CtcjOK7y5w#Connect
        Lib3hClientProtocol::Connect(connect_data) => {
            // ??CHECK??
            // - is this still needed in ghost actor land?

            // short term:
            // this is A:
            // if B in table then connected success!
            // return Lib3hServerProtocol::Connected(peerB_uri) to A core
            // else
            // return Lib3hServerProtocol::FailureResult to A core

            // long term:
            // this is A:
            // check if B enabled in space
            //  if not Lib3hClientProtocol::FailureResult to A core
            // put A -> B Lib3hClientProtocol::Connect(peerB_uri) in db
            connect_data;
        },

        // https://hackmd.io/Rag5au4dQfm1CtcjOK7y5w#JoinSpace
        Lib3hClientProtocol::JoinSpace(space_data) => {
            //   create table if not exists
            //   enable self in table and dirty poll
            // return if no db error
            //   - Lib3hSeverProtocol::SuccessResult to core
            //   - Lib3hServerProtocol::HandleGetAuthoringEntryList to core
            //   - Lib3hServerProtocol::HandleGetGossipingEntryList to core
            // return Lib3hClientProtocol::FailureResult if there is a db error
        },
        Lib3hClientProtocol::LeaveSpace(space_data) => {
            // short term:
            // disable self in table
            // flush all dirty polls

            // long term: cancel outstanding polling e.g. for connections or whatever
        },
        Lib3hClientProtocol::SendDirectMessage(direct_message_data) => {
            // this is A:
            // we put the message in the database for B
            // start a dirty poll for Lib3hClientProtocol::HandleSendDirectMessageResult
        },
        Lib3hClientProtocol::HandleSendDirectMessageResult(direct_message_data) => {
            // this is A:
            // dirty poll the db to see if there is a pending result from B
            // stop the dirty poll
            // pass it on to core
         },

        Lib3hClientProtocol::FetchEntry(fetch_entry_data) => {
            // this is A:
            // get from db
            //   send Lib3hSeverProtocol::FetchEntryResult to A core
         },
        Lib3hClientProtocol::HandleFetchEntryResult(fetch_entry_result_data) => {
            // short term:
            // never going to happen

            fetch_entry_result_data;
        },

        Lib3hClientProtocol::PublishEntry(provided_entry_data) => {

            // this is A:
            // put in db
            // this includes both an Entry and EntryAspects

             provided_entry_data;

        },
        Lib3hClientProtocol::HoldEntry(provided_entry_data) => {
            // short term:
            // this never happens we assume local validation is enough

            // long term:
            // some kind of query to do neighbourhoods

            provided_entry_data;
        },
        Lib3hClientProtocol::QueryEntry(query_entry_data) => {
            // ?? CHECK ??
            // - see what this is about

            query_entry_data;
        },
        Lib3hClientProtocol::HandleQueryEntryResult(query_entry_result_data) => {
            // short term:
            // this never happens
             query_entry_result_data; },

        Lib3hClientProtocol::HandleGetAuthoringEntryListResult(entry_list_data) => {
            // ??CHECK??
            // - what is needed short/long term?
            entry_list_data; },
        Lib3hClientProtocol::HandleGetGossipingEntryListResult(entry_list_data) => {
            // short term:
            // this never happens
            entry_list_data; },
    }
}

/// mimic lib3h::engine::real_engine::serve_Lib3hClientProtocol
pub fn serve_Lib3hServerProtocol(client_msg: Lib3hClientProtocol) {
    debug!("serving: {:?}", client_msg);

    /// docs for all sequences at:
    /// https://hackmd.io/Rag5au4dQfm1CtcjOK7y5w
    match protocol {
        pub enum Lib3hServer(InFromNetwork)Protocol {

            // this doesn't do anything standalone
            SuccessResult(GenericResultData),

            // this doesn't do anything standalone
            FailureResult(GenericResultData),

            Connected(ConnectedData) {

                // short term:
                // this never happens! it's just returned to A if B in db

                // ???CHECK???
                // - what to do when a connection fails?
                // - what happens if A and B disagree on connection state (B thinks it is connected at the exact moment A times out)

                // long term:
                // this is B:
                // something in B sees the A -> B Lib3hClientProtocol::Connect(peerB_uri) in the db
                //   check if A dirty polled recently in space
                //     if not Lib3hClientProtocol::FailureResult to B core
                //   B puts A -> B Lib3hServerProtocol::Connected(peerB_uri) in the db
                //   B sends A -> B Lib3hServerProtocol::Connected(peerA_uri) to B's core

                // this is A:
                // something in A sees the A -> B Lib3hServerProtocol::Connected(peerB_uri) within TIMEOUT
                //   A sends Lib3hServerProtocol::Connected(peerB_uri) to A's core
                // else there was a TIMEOUT so
                //   A sends Lib3hClientProtocol::FailureResult to A's core

            }

            Disconnected(DisconnectedData) {
                // short term:
                // - this can't happen because connect doesn't happen
                // - at least it would be a no-op
            }

            SendDirectMessageResult(DirectMessageData), {
                // this is B:
                // put the result for A back in the db for A's dirty poll to discover

            }

            HandleSendDirectMessage(DirectMessageData), {
                // this B:
                // something has put a pending message for us in the db
                // forward it on to core
            }

            FetchEntryResult(FetchEntryResultData) {
                // this is A
                // this is what is returned to core with an entry in it hopefully
            }

            HandleFetchEntry(FetchEntryData) {
                // short term:
                // this never happens because data magically always comes from "someone else"

                // long term:
                // trigger this when people query the db
            }

            HandleStoreEntryAspect(StoreEntryAspectData) {
                // short term:
                // never going to happen as aspects live in the db
            }

            HandleDropEntry(DropEntryData) {
                // short term:
                // not going to happen as there are no arcs

                // ?? CHECK ??
                // - confirm whether we require this for deletion of entry aspects
            }

            HandleQueryEntry(QueryEntryData) {
                // ?? CHECK ??
            }

            QueryEntryResult(QueryEntryResultData) {
                // ?? CHECK ??
            }

            HandleGetAuthoringEntryList(GetListData) {
                // ?? CHECK ??
            }

            HandleGetGossipingEntryList(GetListData) {
                // this doesn't happen
            }

            // n3h specific
            Terminated,

            // n3h specific
            P2pReady,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rusoto_core::Region;
    use dynomite::{
        dynamodb::{
            DynamoDb, DynamoDbClient, ListTablesInput
        },
        retry::Policy,
        Retries,
    };
    use tokio::runtime::Runtime;
    use rusoto_dynamodb::ListTablesOutput;

    #[test]
    /// we should be able to open up a connection to the local db and find it empty
    fn local_connection_test() {
        let mut rt = Runtime::new().expect("failed to initialize futures runtime");
        let client = DynamoDbClient::new(Region::Custom {
            name: "us-east-1".into(),
            endpoint: "http://localhost:8000".into(),
        })
        .with_retries(Policy::default());

        let list_tables_input: ListTablesInput = Default::default();

        let foo = rt.block_on(client.list_tables(list_tables_input));

        assert_eq!(Ok(ListTablesOutput { last_evaluated_table_name: None, table_names: Some([].to_vec()) }), foo);

    }
}
