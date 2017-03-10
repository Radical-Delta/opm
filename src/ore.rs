use ::chrono::prelude::*;
use ::std::io::{Result, Read};
use ::hyper::*;
use ::hyper::client::*;
use ::json;
use ::json::*;

#[derive(Debug, Clone, Copy)]
pub enum PluginCategory {
    AdminTools, Chat, DeveloperTools, Economy, Gameplay, Games, Protection, RolePlaying, WorldManagement, Miscellaneous
}

impl PluginCategory {
    fn to_int(&self) -> u8 {
        match *self {
            PluginCategory::AdminTools => 0,
            PluginCategory::Chat => 1,
            PluginCategory::DeveloperTools => 2,
            PluginCategory::Economy => 3,
            PluginCategory::Gameplay => 4,
            PluginCategory::Games => 5,
            PluginCategory::Protection => 6,
            PluginCategory::RolePlaying => 7,
            PluginCategory::WorldManagement => 8,
            PluginCategory::Miscellaneous => 9,
        }
    }
    fn from_int(id: u8) -> Self {
        match id {
            0 => PluginCategory::AdminTools,
            1 => PluginCategory::Chat,
            2 => PluginCategory::DeveloperTools,
            3 => PluginCategory::Economy,
            4 => PluginCategory::Gameplay,
            5 => PluginCategory::Games,
            6 => PluginCategory::Protection,
            7 => PluginCategory::RolePlaying,
            8 => PluginCategory::WorldManagement,
            9 => PluginCategory::Miscellaneous,
            _ => unreachable!(),
        }
    }
    fn from_str(id: &str) -> Self {
        match id {
            "Admin Tools" => PluginCategory::AdminTools,
            "Chat" => PluginCategory::Chat,
            "Developer Tools" => PluginCategory::DeveloperTools,
            "Economy" => PluginCategory::Economy,
            "Gameplay" => PluginCategory::Gameplay,
            "Games" => PluginCategory::Games,
            "Protection" => PluginCategory::Protection,
            "Role Playing" => PluginCategory::RolePlaying,
            "World Management" => PluginCategory::WorldManagement,
            "Miscellaneous" => PluginCategory::Miscellaneous,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortType {
    RecentlyUpdated, MostStars, MostDownloads, MostViews, Newest
}

impl SortType {
    fn to_int(&self) -> u8 {
        match *self {
            SortType::MostStars => 0,
            SortType::MostDownloads => 1,
            SortType::MostViews => 2,
            SortType::Newest => 3,
            SortType::RecentlyUpdated => 4,
        }
    }
    fn from_int(id: u8) -> Self {
        match id {
            0 => SortType::MostStars,
            1 => SortType::MostDownloads,
            2 => SortType::MostViews,
            3 => SortType::Newest,
            4 => SortType::RecentlyUpdated,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearchQuery<'a> {
    categories: Option<Vec<PluginCategory>>,
    sort: Option<SortType>,
    query: &'a str,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> SearchQuery<'a> {
    pub fn set_categories(&mut self, categories: &Vec<PluginCategory>) -> &mut Self {
        self.categories = Some(categories.clone());
        self
    }
    pub fn set_sort_type(&mut self, sort_type: SortType) -> &mut Self {
        self.sort = Some(sort_type);
        self
    }
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn set_offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn exec<'b>(self) -> Vec<Plugin<'b>> {
        let client = Client::new();
        let mut url = Url::parse("https://ore.spongepowered.org/api/projects").unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(categories) = self.categories {
                pairs.append_pair("categories", categories.into_iter().map(|x| x.to_int()).fold(String::new(), |x, y| x + "," + y.to_string().as_str()).as_str());
            }
            if let Some(sort) = self.sort {
                pairs.append_pair("sort", sort.to_int().to_string().as_str());
            }
            if let Some(limit) = self.limit {
                pairs.append_pair("limit", limit.to_string().as_str());
            }
            if let Some(offset) = self.offset {
                pairs.append_pair("offset", offset.to_string().as_str());
            }
            pairs.append_pair("q", self.query);
        }
        let mut plugins: Vec<Plugin> = Vec::new();

        let mut res = String::new();
        client.get(url).send().unwrap().read_to_string(&mut res);
        let result = json::parse(res.as_str()).unwrap();
        for plugin in result.members() {
            plugins.push(parse_plugin(plugin));
        }

        plugins
    }
}

pub fn search(query: &str) -> SearchQuery {
    SearchQuery {
        categories: Some(Vec::new()),
        sort: Some(SortType::RecentlyUpdated),
        query: query,
        limit: Some(25),
        offset: Some(0),
    }
}

fn parse_plugin<'a>(plugin: &'a JsonValue) -> Plugin<'a> {
    Plugin {
        plugin_id: plugin["pluginId"].as_str().unwrap(),
        created_at: parse_date(&plugin["createdAt"].as_str().unwrap()),
        name: plugin["name"].as_str().unwrap(),
        owner: plugin["owner"].as_str().unwrap(),
        description: plugin["description"].as_str().unwrap(),
        href: plugin["href"].as_str().unwrap(),
        members: plugin["members"].members().into_iter().map(parse_user).fold(Vec::new(), |mut v, f| {v.push(f); v}),
        channels: plugin["channels"].members().into_iter().map(parse_channel).fold(Vec::new(), |mut v, f| {v.push(f); v}),
        recommended: parse_version(&plugin["recommended"]),
        category: PluginCategory::from_str(plugin["category"]["title"].as_str().unwrap()),
        views: plugin["views"].as_u32().unwrap(),
        downloads: plugin["downloads"].as_u32().unwrap(),
        stars: plugin["stars"].as_u32().unwrap(),
    }
}

fn parse_date(s: &str) -> DateTime<UTC> {
    unimplemented!()
}

fn parse_user<'a>(user: &'a JsonValue) -> User<'a> {
    User {
        user_id: user["userId"].as_u32().unwrap(),
        name: user["name"].as_str().unwrap(),
        roles: user["roles"].members().into_iter().map(|role| role.as_str().unwrap()).fold(Vec::new(), |mut v, f| {v.push(f); v}),
        head_role: user["headRole"].as_str().unwrap(),
    }
}

fn parse_channel<'a>(channel: &'a JsonValue) -> Channel<'a> {
    Channel {
        name: channel["name"].as_str().unwrap(),
        color: channel["color"].as_str().unwrap(),
    }
}

fn parse_version<'a>(version: &'a JsonValue) -> Version<'a> {
    Version {
        id: version["id"].as_u32().unwrap(),
        created_at: parse_date(version["createdAt"].as_str().unwrap()),
        name: version["name"].as_str().unwrap(),
        dependencies: version["dependencies"].members().into_iter().map(parse_dependency).fold(Vec::new(), |mut v, f| {v.push(f); v}),
        plugin_id: version["pluginId"].as_str().unwrap(),
        channel: parse_channel(&version["channel"]),
        file_size: version["fileSize"].as_u32().unwrap(),
    }
}

fn parse_dependency<'a>(dependency: &'a JsonValue) -> Dependency<'a> {
    Dependency {
        plugin_id: dependency["pluginId"].as_str().unwrap(),
        version: dependency["version"].as_str().unwrap(),
    }
}

#[derive(Clone, Debug)]
pub struct Plugin<'a> {
    pub plugin_id: &'a str,
    pub created_at: DateTime<UTC>,
    pub name: &'a str,
    pub owner: &'a str,
    pub description: &'a str,
    pub href: &'a str,
    pub members: Vec<User<'a>>,
    pub channels: Vec<Channel<'a>>,
    pub recommended: Version<'a>,
    pub category: PluginCategory,
    pub views: u32,
    pub downloads: u32,
    pub stars: u32,
}

#[derive(Clone, Debug)]
pub struct User<'a> {
    pub user_id: u32,
    pub name: &'a str,
    pub roles: Vec<&'a str>,
    pub head_role: &'a str,
}

#[derive(Clone, Copy, Debug)]
pub struct Channel<'a> {
    pub name: &'a str,
    pub color: &'a str,
}

#[derive(Clone, Debug)]
pub struct Version<'a> {
    pub id: u32,
    pub created_at: DateTime<UTC>,
    pub name: &'a str,
    pub dependencies: Vec<Dependency<'a>>,
    pub plugin_id: &'a str,
    pub channel: Channel<'a>,
    pub file_size: u32,
}

#[derive(Clone, Debug)]
pub struct Dependency<'a> {
    pub plugin_id: &'a str,
    pub version: &'a str,
}