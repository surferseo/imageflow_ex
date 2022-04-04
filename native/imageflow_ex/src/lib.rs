extern crate imageflow_types;

use rustler::{Binary, Encoder, Env, Term, NifResult};

mod job;

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

rustler::init!(
    "Elixir.Imageflow.NIF",
    [
        get_long_version_string,
        job_create,
        job_destroy,
        job_add_input_buffer,
        job_add_input_file,
        job_add_output_buffer,
        job_get_output_buffer,
        job_save_output_to_file,
        job_message,
    ]
);

use job::Job;


macro_rules! job {
    ($id:expr) => {{
        Job::load_from_id($id).ok().unwrap()
    }};
}

#[rustler::nif()]
fn get_long_version_string<'a>(env: Env<'a>) -> String {
    imageflow_types::version::one_line_version()
}

#[rustler::nif()]
fn job_create<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    match Job::create() {
        Ok(id) => Ok((atoms::ok(), id).encode(env)),
        Err(_e) => Err(rustler::Error::Atom("Unable to create context")),
    }
}

#[rustler::nif()]
fn job_destroy<'a>(env: Env<'a>, id: usize) -> NifResult<Term<'a>> {
    Job::destroy_from_id(id).ok().unwrap();

    Ok(atoms::ok().encode(env))
}

#[rustler::nif()]
fn job_add_input_buffer<'a>(env: Env<'a>, id: usize, io_id: i32, bytes: Binary) -> NifResult<Term<'a>> {
    match job!(id).add_input_buffer(io_id, bytes.as_slice()) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(msg) => Ok((atoms::error(), msg).encode(env)),
    }
}

#[rustler::nif()]
fn job_add_input_file<'a>(env: Env<'a>, id: usize, io_id: i32, path: String) -> NifResult<Term<'a>> {
    match job!(id).add_input_file(io_id, &path) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(msg) => Ok((atoms::error(), msg).encode(env)),
    }
}

#[rustler::nif()]
fn job_add_output_buffer<'a>(env: Env<'a>, id: usize, io_id: i32) -> NifResult<Term<'a>> {
    match job!(id).add_output_buffer(io_id) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(msg) => Ok((atoms::error(), msg).encode(env)),
    }
}

#[rustler::nif()]
fn job_get_output_buffer<'a>(env: Env<'a>, id: usize, io_id: i32) -> NifResult<Term<'a>> {
    match job!(id).get_output_buffer(io_id) {
        Ok(buffer) => Ok((atoms::ok(), buffer).encode(env)),
        Err(e) => Ok((atoms::error(), e.to_string()).encode(env)),
    }
}

#[rustler::nif()]
fn job_save_output_to_file<'a>(env: Env<'a>, id: usize, io_id: i32, path: String) -> NifResult<Term<'a>> {
    match job!(id).save_output_to_file(io_id, &path) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => Ok((atoms::error(), e.to_string()).encode(env)),
    }
}

#[rustler::nif()]
fn job_message<'a>(env: Env<'a>, id: usize, method: String, message: String) -> NifResult<Term<'a>> {
    match job!(id).message(&method, &message) {
        Ok(resp) => Ok((atoms::ok(), resp.response_json.encode(env)).encode(env)),
        Err(msg) => Ok((atoms::error(), msg.encode(env)).encode(env)),
    }
}
