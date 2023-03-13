use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use log::info;
use rouille::{Request, Response, router};

use mvc::hash_map_model::HashMapModel;
use mvc::simple_controller::SimpleController;
use mvc::simple_viewer::SimpleViewer;
use mvc::task_controller::TaskController;

type NaiveController = SimpleController<HashMapModel, SimpleViewer>;

fn handle_index(controller: &Mutex<NaiveController>, _: &Request) -> Response {
    match controller.lock().unwrap().render_index() {
        Ok(txt) => {
            info!("Ok, rendered, return html {}", txt);
            Response::html(txt)
        }
        Err(err) => {
            info!("Err, failed to render, return error text");
            Response::text(err)
        }
    }
}

fn generic_handle_with_redirect<O, E>(value: Result<O, E>) -> Response
    where String: From<E> {
    match value {
        Ok(_) => {
            info!("Ok, return index");
            Response::redirect_302("/")
        }
        Err(err) => {
            info!("Err, return text");
            Response::text(err)
        }
    }
}

fn handle_create_task(controller: &Mutex<NaiveController>, req: &Request) -> Response {
    info!("url: {}", req.url());
    generic_handle_with_redirect(
        controller
            .lock()
            .unwrap()
            .create_new_undone_task(
                req
                    .get_param("name")
                    .unwrap_or(String::from("no name")),
                req
                    .get_param("statement")
                    .unwrap_or(String::from("no statement")),
                req
                    .get_param("list")
                    .unwrap_or(String::from("no list"))))
}

fn handle_update_task(controller: &Mutex<NaiveController>, req: &Request) -> Response {
    info!("url: {}", req.url());
    generic_handle_with_redirect(
        controller
            .lock()
            .unwrap()
            .update_task_done(
                req
                    .get_param("task_id")
                    .unwrap()
                    .parse()
                    .unwrap()))
}

fn handle_delete_task_list(controller: &Mutex<NaiveController>, req: &Request) -> Response {
    info!("url: {}", req.url());
    generic_handle_with_redirect(
        controller
            .lock()
            .unwrap()
            .delete_task_list(
                req
                    .get_param("list_id")
                    .unwrap()
                    .parse()
                    .unwrap()))
}

fn main() {
    env_logger::init();
    let server_address = "localhost:1337";

    let model = Box::new(HashMapModel::new());

    let mut template_buf = vec![];
    File::open("src/resources/index.template")
        .expect("Failed to open resources")
        .read_to_end(&mut template_buf)
        .expect("Failed to read resources");
    let template_buf = String::from_utf8(template_buf)
        .expect("Broken utf-8 in resources");
    let viewer = Box::new(SimpleViewer::new(template_buf));

    let controller = Mutex::new(NaiveController { model, viewer });

    info!("Starting server on {}", server_address);
    rouille::start_server(server_address, move |req| {
        router!(req,
            (GET) (/) => {
                info!("Serve index: {:?}", req);
                handle_index(&controller, req)
            },
            (GET) (/create_task) => {
                info!("Serve create_task: {:?}", req);
                handle_create_task(&controller, req)
            },
            (GET) (/update_task_done) => {
                info!("Serve update_task: {:?}", req);
                handle_update_task(&controller, req)
            },
            (GET) (/delete_task_list) => {
                info!("Serve delete list: {:?}", req);
                handle_delete_task_list(&controller, req)
            },
            _ => {
                info!("Unknown request: {:?}", req);
                Response::empty_404()
            }
        )
    });
}
