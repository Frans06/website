use leptos::prelude::*;

#[component]
pub fn Background() -> impl IntoView {
    view! {  <div class="stars background-surface">
          {(0..50).map(|i| view! {
              <div
                  class="star"
                  style=format!(
                      "left: {}%; top: {}%; width: {}px; height: {}px; animation-delay: {}s;",
                      (i * 17) % 100,
                      (i * 23) % 100,
                      1 + (i % 3),
                      1 + (i % 3),
                      (i as f32) * 0.1
                  )
              ></div>
          }).collect::<Vec<_>>()}
      </div>
    }
}
