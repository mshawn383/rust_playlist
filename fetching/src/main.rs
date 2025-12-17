use leptos::component;
use leptos::leptos_dom::logging;
use leptos::prelude::*;
use leptos::svg::view;
use reqwasm::http::*;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Company {
    pub name: String,
    pub catchPhrase: String,
    pub bs: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Post {
    pub userId: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}
#[derive(Debug, thiserror::Error)]
enum FetchError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

async fn fetch_fake() -> Result<Vec<Post>, String> {
    let res = reqwasm::http::Request::get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
       .unwrap()
        .json::<Vec<Post>>()
        .await
        .map_err(|e| e.to_string())?;


    // log_1(&format!("Users received: {:?}", res).into());

    Ok(res)
}

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    // start the resource
    let res: LocalResource<Result<Vec<Post>, String>> =
        LocalResource::new( move || fetch_fake());
let (user_data,set_user_data)=signal::<Vec<Post>>(vec![]);
    let data: LocalResource<Result<String, ()>> =
        LocalResource::new(|| async { Ok::<_, ()>("Hello from async!".to_string()) });


        Effect::new(move |_| {
        if let Some(Ok(posts)) = res.get() {
            set_user_data.set(posts);   
        }
});
    view! {

        // <Transition fallback=view! {<p>"Loading posts..."</p>}>
        //      {move || Suspend::new(async move {
        //         res.await.map(|posts| {
        //             view! {
        //                 <div>
        //                     <h1>"Posts"</h1>
        //                     <ul>
        //                         {posts.into_iter().map(|post| {
        //                             view! {
        //                                 <li >
        //                                     <h3>{post.title}</h3>
        //                                     <p>{post.body}</p>
        //                                 </li>
        //                             }
        //                         }).collect::<Vec<_>>()}
        //                     </ul>
        //                 </div>
        //             }
        //         })
        //      })}
        // </Transition>
       
       <For
       each=move || user_data.get()
         key=|post| post.id
         children=|post| view! {
            <div>
                <h2>{post.title.clone()}</h2>
                <p>{post.body.clone()}</p>
            </div>
         }
       />
    }
}
