use cot_site::{GuideItem, cot_site_common, md_page};

/// Documentation areas and page groups. URLs do not depend on sidebar placement.
pub fn documentation() -> Vec<(&'static str, Vec<GuideItem>)> {
    vec![
        (
            "Start here",
            vec![
                GuideItem::Page(md_page!("start")),
                GuideItem::Page(md_page!("installation")),
                GuideItem::Page(md_page!("learning-paths")),
                GuideItem::Page(md_page!("introduction")),
            ],
        ),
        (
            "Tutorials",
            vec![
                GuideItem::Page(md_page!("tutorials/overview")),
                GuideItem::SubCategory {
                    title: "Your first application",
                    pages: vec![
                        md_page!("tutorials/first-app"),
                        md_page!("tutorials/model"),
                        md_page!("tutorials/views"),
                        md_page!("tutorials/forms-auth"),
                        md_page!("tutorials/test-deploy"),
                    ],
                },
                GuideItem::Page(md_page!("tutorials/json-api")),
                GuideItem::Page(md_page!("tutorials/reusable-app")),
            ],
        ),
        (
            "Guides",
            vec![
                GuideItem::Page(md_page!("guides/overview")),
                GuideItem::SubCategory {
                    title: "Fundamentals",
                    pages: vec![
                        md_page!("guides/projects"),
                        md_page!("guides/lifecycle"),
                        md_page!("guides/configuration"),
                        md_page!("guides/async-state"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Requests and responses",
                    pages: vec![
                        md_page!("routing"),
                        md_page!("guides/requests"),
                        md_page!("guides/responses"),
                        md_page!("guides/middleware"),
                        md_page!("guides/sessions"),
                        md_page!("guides/errors"),
                        md_page!("error-pages"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Pages and forms",
                    pages: vec![
                        md_page!("templates"),
                        md_page!("forms"),
                        md_page!("guides/validation"),
                        md_page!("static-files"),
                        md_page!("guides/media"),
                        md_page!("guides/localization"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Databases",
                    pages: vec![
                        md_page!("databases/overview"),
                        md_page!("guides/relationships"),
                        md_page!("databases/queries"),
                        md_page!("guides/pagination"),
                        md_page!("databases/transactions"),
                        md_page!("databases/migrations"),
                        md_page!("guides/seeding"),
                        md_page!("guides/query-performance"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Authentication and security",
                    pages: vec![
                        md_page!("guides/authentication"),
                        md_page!("guides/account-lifecycle"),
                        md_page!("guides/authorization"),
                        md_page!("guides/web-security"),
                        md_page!("guides/rate-limiting"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Application services",
                    pages: vec![
                        md_page!("caching"),
                        md_page!("sending-emails"),
                        md_page!("guides/background-tasks"),
                        md_page!("guides/scheduling"),
                        md_page!("guides/events"),
                        md_page!("admin-panel"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "APIs and integrations",
                    pages: vec![
                        md_page!("guides/json-apis"),
                        md_page!("openapi"),
                        md_page!("guides/http-webhooks"),
                        md_page!("guides/realtime"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Testing",
                    pages: vec![
                        md_page!("testing"),
                        md_page!("guides/http-tests"),
                        md_page!("guides/database-tests"),
                        md_page!("guides/browser-tests"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Deployment and operations",
                    pages: vec![
                        md_page!("guides/deployment"),
                        md_page!("guides/production"),
                        md_page!("guides/observability"),
                        md_page!("guides/performance"),
                        md_page!("guides/recovery"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Extending Cot",
                    pages: vec![
                        md_page!("guides/reusable-apps"),
                        md_page!("guides/custom-components"),
                        md_page!("guides/management-commands"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Coming from another framework",
                    pages: vec![
                        md_page!("coming-from/overview"),
                        md_page!("coming-from/rust-for-web-developers"),
                        md_page!("coming-from/django"),
                        md_page!("coming-from/laravel"),
                        md_page!("coming-from/rails"),
                        md_page!("coming-from/spring-boot"),
                        md_page!("coming-from/axum"),
                        md_page!("coming-from/actix-web"),
                    ],
                },
            ],
        ),
        (
            "How-to guides",
            vec![
                GuideItem::Page(md_page!("how-to/overview")),
                GuideItem::SubCategory {
                    title: "Application development",
                    pages: vec![
                        md_page!("how-to/diagnose-routing"),
                        md_page!("how-to/private-uploads"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Testing and deployment",
                    pages: vec![
                        md_page!("how-to/test-redirect"),
                        md_page!("how-to/production-build"),
                    ],
                },
                GuideItem::SubCategory {
                    title: "Application services",
                    pages: vec![md_page!("how-to/queued-email")],
                },
            ],
        ),
        (
            "Reference",
            vec![
                GuideItem::Page(md_page!("reference/overview")),
                GuideItem::Page(md_page!("reference/configuration")),
                GuideItem::Page(md_page!("reference/cli")),
                GuideItem::Page(md_page!("reference/features")),
                GuideItem::Page(md_page!("reference/components")),
            ],
        ),
        (
            "Releases",
            vec![
                GuideItem::Page(md_page!("releases/overview")),
                GuideItem::Page(md_page!("upgrade-guide")),
            ],
        ),
        (
            "Blog",
            vec![
                GuideItem::Page(md_page!("blog/overview")),
                GuideItem::Page(md_page!("blog/framework-transition-guides")),
            ],
        ),
        (
            "Community",
            vec![
                GuideItem::Page(md_page!("community/overview")),
                GuideItem::Page(md_page!("framework-comparison")),
            ],
        ),
    ]
}
