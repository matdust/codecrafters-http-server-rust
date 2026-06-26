use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{handler::Handler, request::HttpMethod};

#[derive(Debug)]
pub struct Router<'a> {
    routes: Node<'a>,
}

impl<'a> Default for Router<'a> {
    fn default() -> Self {
        Self {
            routes: Node {
                static_routes: HashMap::new(),
                dynamic_route: None,
                handlers: HashMap::new(),
            },
        }
    }
}

impl<'a> Router<'a> {
    // {value} - is treated as dynamic route
    pub fn add_route(
        &mut self,
        http_method: HttpMethod,
        route: &str,
        handler: &'a dyn Handler,
    ) -> anyhow::Result<()> {
        // TODO: add check for route
        let parts = route.split('/').skip(1).collect::<Vec<_>>();
        let mut path_parts_iter = parts.into_iter().peekable();
        let mut head = &mut self.routes;

        for path_part in &mut path_parts_iter {
            if head.get_static_route_mut(path_part).is_some() {
                head = head.get_static_route_mut(path_part).unwrap();
                continue;
            }

            if Node::is_dynamic_route_part(path_part) && head.get_dynamic_route_mut().is_some() {
                head = head.get_dynamic_route_mut().unwrap();
                continue;
            }

            // add current path_part
            if Node::is_dynamic_route_part(path_part) {
                if head.dynamic_route.is_none() {
                    if path_parts_iter.peek().is_none() {
                        head.set_dynamic_route(path_part);
                        return head
                            .get_dynamic_route_mut()
                            .unwrap_or_else(|| panic!("dynamic route {} is not set", route))
                            .set_handler(http_method, handler);
                    } else {
                        head.set_dynamic_route(path_part);
                        return self.add_route(http_method, route, handler);
                    }
                } else {
                    return Err(anyhow::Error::msg(format!(
                        "cannot set route {} because it's already set",
                        &route
                    )));
                }
            } else {
                // if node is present and doesn't have handler
                if head.static_routes.contains_key(path_part) {
                    let static_route = head.static_routes.get_mut(path_part).unwrap();
                    if !static_route.handlers.contains_key(&http_method) {
                        return static_route.set_handler(http_method.clone(), handler);
                    }
                }
                // if its a last part of path
                else if path_parts_iter.peek().is_none() {
                    head.set_static_route(path_part);
                    return head
                        .get_static_route_mut(path_part)
                        .unwrap_or_else(|| panic!("static route {} is not set", path_part))
                        .set_handler(http_method.clone(), handler);
                } else {
                    head.set_static_route(path_part);
                    return self.add_route(http_method, route, handler);
                }
            }
        }

        if !head.handlers.contains_key(&http_method) {
            head.set_handler(http_method.clone(), handler)?;
        }

        Ok(())
    }

    pub fn match_route(&self, http_method: HttpMethod, route: &str) -> Option<&'a dyn Handler> {
        let parts = route.split('/').skip(1).collect::<Vec<_>>();
        let mut head = &self.routes;

        for path_part in parts {
            if let Some(static_route) = head.static_routes.get(path_part) {
                head = static_route;
                continue;
            }

            if let Some(dynamic_route) = &head.dynamic_route {
                head = dynamic_route.1.as_ref();
                continue;
            }

            return None;
        }

        head.handlers.get(&http_method).copied()
    }
}

#[derive(Clone, Default)]
struct Node<'a> {
    // /foo/bar
    static_routes: HashMap<String, Node<'a>>,
    // /{foo}/{bar}
    dynamic_route: Option<(String, Box<Node<'a>>)>,
    handlers: HashMap<HttpMethod, &'a dyn Handler>,
}

impl<'a> Node<'a> {
    pub fn set_static_route(&mut self, path: &str) -> &mut Self {
        self.static_routes.insert(path.to_string(), Node::default());
        self
    }
    pub fn set_dynamic_route(&mut self, path: &str) -> &mut Self {
        self.dynamic_route = Some((path.to_string(), Box::new(Node::default())));
        self
    }

    pub fn set_handler(
        &mut self,
        http_method: HttpMethod,
        handler: &'a dyn Handler,
    ) -> anyhow::Result<()> {
        if self.handlers.contains_key(&http_method) {
            return Err(anyhow::Error::msg("key alread exists"));
        }
        self.handlers.insert(http_method, handler);
        Ok(())
    }

    pub fn get_static_route_mut(&mut self, path_part: &str) -> Option<&mut Self> {
        self.static_routes.get_mut(path_part)
    }

    pub fn get_dynamic_route_mut(&mut self) -> Option<&mut Self> {
        Some(self.dynamic_route.as_mut()?.1.deref_mut())
    }

    pub fn is_dynamic_route_part(path_part: &str) -> bool {
        path_part.starts_with('{') && path_part.ends_with('}')
    }
}

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("static_routes", &self.static_routes)
            .field("dynamic_route", &self.dynamic_route)
            .field("handlers", &self.handlers)
            .finish()
    }
}
