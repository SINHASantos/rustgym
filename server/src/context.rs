use crate::session_data::SessionData;
use crate::AppData;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::HttpResponse;
use askama::Template;
use rustgym_schema::adventofcode_description::AdventOfCodeDescription;
use rustgym_schema::adventofcode_solution::AdventOfCodeSolution;
use rustgym_schema::google_problem::GoogleProblem;
use rustgym_schema::leetcode_description::LeetcodeDescription;
use rustgym_schema::leetcode_question::LeetcodeQuestion;
use rustgym_schema::leetcode_solution::LeetcodeSolution;

pub struct AppContext {
    pub title: String,
    pub tag: String,
}

#[derive(Template, new)]
#[template(path = "home.j2")]
pub struct HomeContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
}

impl AppContext {
    pub fn new(data: Data<AppData>) -> Self {
        let title = data.title.borrow().to_string();
        let tag = data.tag.borrow().to_string();
        AppContext { title, tag }
    }
}

#[derive(Queryable)]
pub struct LeetcodeIndexRow {
    pub id: i32,
    pub title: String,
    pub level: i32,
}

impl LeetcodeIndexRow {
    pub fn level_str(&self) -> &str {
        match self.level {
            1 => "Easy",
            2 => "Medium",
            3 => "Hard",
            _ => "",
        }
    }

    pub fn href(&self) -> String {
        format!("/leetcode/{}", self.id)
    }
}

#[derive(Template, new)]
#[template(path = "leetcode-index.j2")]
pub struct LeetcodeIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows: Vec<LeetcodeIndexRow>,
}

#[derive(Queryable)]
pub struct AdventOfCodeIndexRow {
    pub id: i32,
    pub year: i32,
    pub day: i32,
    pub title: String,
}

#[derive(Template, new)]
#[template(path = "adventofcode-index.j2")]
pub struct AdventOfCodeIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub rows: Vec<AdventOfCodeIndexRow>,
}

impl AdventOfCodeIndexRow {
    pub fn href(&self) -> String {
        format!("/adventofcode/{}", self.id)
    }
}

#[derive(Queryable, Clone)]
pub struct GoogleIndexRow {
    pub id: i32,
    pub division: i32,
    pub year: i32,
    pub round: i32,
    pub title: String,
}

impl GoogleIndexRow {
    pub fn href(&self) -> String {
        format!("/google/{}", self.id)
    }
}

#[derive(Template, new)]
#[template(path = "google-index.j2")]
pub struct GoogleIndexContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub codejam_rows: Vec<GoogleIndexRow>,
    pub kickstart_rows: Vec<GoogleIndexRow>,
}

#[derive(Template, new)]
#[template(path = "leetcode-detail.j2")]
pub struct LeetcodeDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub question: LeetcodeQuestion,
    pub description: LeetcodeDescription,
    pub solutions: Vec<LeetcodeSolution>,
}

#[derive(Template, new)]
#[template(path = "adventofcode-detail.j2")]
pub struct AdventOfCodeDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub description: AdventOfCodeDescription,
    pub solution: AdventOfCodeSolution,
}

#[derive(Template, new)]
#[template(path = "google-detail.j2")]
pub struct GoogleDetailContext {
    pub app: AppContext,
    pub session: SessionData,
    pub path: String,
    pub item: GoogleProblem,
}

#[derive(Template, new)]
#[template(path = "sitemap.j2")]
pub struct SitemapContext {
    pub leetcode_rows: Vec<LeetcodeDescription>,
    pub adventofcode_rows: Vec<AdventOfCodeDescription>,
}

#[derive(Template, new)]
#[template(path = "robots.j2")]
pub struct RobotsContext {}

macro_rules! impl_render_wrapper {
    ($type: ty) => {
        impl $type {
            pub fn render_wrapper(&self) -> Result<HttpResponse, Error> {
                let body = self.render().map_err(ErrorInternalServerError)?;
                Ok(HttpResponse::Ok().content_type("text/html").body(body))
            }
        }
    };
}

macro_rules! impl_txt_render_wrapper {
    ($type: ty) => {
        impl $type {
            pub fn render_wrapper(&self) -> Result<HttpResponse, Error> {
                let body = self.render().map_err(ErrorInternalServerError)?;
                Ok(HttpResponse::Ok().content_type("text/txt").body(body))
            }
        }
    };
}

impl_render_wrapper!(HomeContext);
impl_render_wrapper!(LeetcodeIndexContext);
impl_render_wrapper!(AdventOfCodeIndexContext);
impl_render_wrapper!(LeetcodeDetailContext);
impl_render_wrapper!(AdventOfCodeDetailContext);
impl_render_wrapper!(GoogleIndexContext);
impl_render_wrapper!(GoogleDetailContext);

impl_txt_render_wrapper!(SitemapContext);
impl_txt_render_wrapper!(RobotsContext);
