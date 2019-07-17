/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct StoreApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl StoreApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> StoreApiClient {
        StoreApiClient {
            configuration: configuration,
        }
    }
}

pub trait StoreApi {
    fn delete_order(&self, order_id: &str) -> Result<(), Error>;
    fn get_inventory(&self, ) -> Result<::std::collections::HashMap<String, i32>, Error>;
    fn get_order_by_id(&self, order_id: i64) -> Result<::models::Order, Error>;
    fn place_order(&self, body: ::models::Order) -> Result<::models::Order, Error>;
}

impl StoreApi for StoreApiClient {
    fn delete_order(&self, order_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=crate::apis::urlencode(order_id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_inventory(&self, ) -> Result<::std::collections::HashMap<String, i32>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/inventory", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("api_key", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_order_by_id(&self, order_id: i64) -> Result<::models::Order, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn place_order(&self, body: ::models::Order) -> Result<::models::Order, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
