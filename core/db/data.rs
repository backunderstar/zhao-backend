use common::AppResult;
use entity::{article, prelude::*, system, user};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set,
    sqlx::types::chrono::Utc,
};
use util::hash_password;

pub async fn insert_data(db: &DatabaseConnection) -> AppResult<()> {
    let u = User::find().count(db).await?;

    if u == 0 {
        let password = hash_password("123456")?;
        let user = user::ActiveModel {
            role: Set(user::Role::Admin),
            username: Set("admin".to_string()),
            nickname: Set("admin".to_string()),
            password: Set(password),
            email: Set("zhao@zhao.com".to_string()),
            avatar: Set("/default/avatar/diaona.webp".to_string()),
            ..Default::default()
        };

        user.insert(db).await?;

        let a = Article::find().count(db).await?;

        if a == 0 {
            let article = article::ActiveModel {
                title: Set("Hello World".to_string()),
                cover: Set("/default/cover/feiji.jpg".to_string()),
                summary: Set("Hello World".to_string()),
                content: Set("博客网站是一种个人或小团队运营的内容平台，其运营模式主要包括以下几个方面：
1. **内容创作：** 博客网站的核心是内容，运营者需要定期创作高质量、有价值的文章来吸引访问者。因此，博客网站的运营者需要具备一定的专业知识和写作能力。
2. **网站设计与维护：** 博客网站的外观设计和用户体验对于访问者的留存率和忠诚度至关重要。运营者需要关注网站的视觉效果、交互设计、网站速度等方面，以提高用户满意度。
3. **推广与营销：** 博客网站需要建立稳定的访问流量和粉丝群体，为此运营者需要采用各种推广和营销方式，如SEO优化、社交媒体运营、广告投放等。
4. **收入来源：** 博客网站可以通过多种方式获取收入，如广告收入、赞助、付费会员等。运营者需要根据自身情况选择合适的收入来源，并保证收入来源与网站内容和形象的契合度。
## 责任
在博客网站的运营过程中，运营者需要承担以下责任：
1. **知识产权和版权责任：** 运营者应当尊重他人的知识产权和版权，不得侵犯他人的版权和著作权。同时，运营者应当保护自己的知识产权和版权，对于抄袭、盗用等行为进行打击。
2. **内容责任：** 博客网站是公开展示内容的平台，运营者应当确保所发布的内容真实、准确、合法，不得宣扬暴力、色情、恐怖等内容，以及其他违法违规内容。
3. **用户隐私保护：** 运营者应当依据相关法律法规，保护用户的个人隐私信息，不得泄露用户的个人信息。
4. **安全责任：** 运营者应当保证博客网站的安全性，采取措施防范黑客攻击、数据丢失等安全问题。
5. **社会责任：** 博客网站虽然是个人或小团队运营的平台，但其内容对社会和公众有一定的影响力。因此，运营者需要履行相关社会责任，推动社会进步、促进公共利益。".to_string()),
                published: Set(true),
                ..Default::default()
            };

            article.insert(db).await?;
        }
    }

    let s = System::find_by_id(1).one(db).await?;

    if s.is_none() {
        let now = Utc::now().naive_utc();
        let system = system::ActiveModel {
            id: Set(1),
            name: Set("朝而往".to_string()),
            description: Set("一个内容管理系统".to_string()),
            keywords: Set("".to_string()),
            copyright: Set("朝而往".to_string()),
            domain: Set("https://zhao.zhao.com".to_string()),
            logo: Set("/default/logo/logo.png".to_string()),
            created_at: Set(now),
        };

        system.insert(db).await?;
    }

    Ok(())
}
