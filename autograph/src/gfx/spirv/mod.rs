use failure::Error;
use gfx;
use gfx::pipeline::GraphicsPipelineBuilder;
use gfx::pipeline::VertexAttribute;
use gfx::shader;
use gfx::shader::DefaultUniformBinder;
use gfx::shader_interface;
use gl;
use gl::types::*;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
