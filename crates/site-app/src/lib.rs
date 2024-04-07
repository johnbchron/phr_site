use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Style>{include_str!("../style/fonts.css")}</Style>
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <Link rel="preload" href="/fonts/inter.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />

    <Title text="Prayer House Reitnau"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=HomePage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="w-full bg-cyan-50/80 text-cyan-950 px-4">
      <div class="mx-auto container flex flex-col py-12 gap-6 min-h-screen">
        // title block
        <div class="flex flex-col gap-2 px-12">
          <p class="text-5xl font-semibold tracking-tight">"Prayer House Reitnau"</p>
          <p class="text-lg font-semibold">"A place of prayer and intercession in Reitnau, Aargau."</p>
        </div>

        // 3x2 grid
        <div class="flex flex-col sm:grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 grid-rows-8 sm:grid-rows-4 md:grid-rows-3 grid-flow-row-dense gap-4 auto-rows-min">
          <TextBox title="Values">
            <p class="leading-tight">
              "We love prayer. Prayer is essential. Prayer is meeting God. Through prayer we truly enter into our calling as priests. Prayer is one of the most beautiful things in life. Prayer is getting to a point where you focus on hearing God, and you change things. Through prayer you open up a space in your life for the Holy Spirit to work in you."
            </p>
          </TextBox>
          <TextBox title="Vision" extra_class="row-span-2 col-span-1 sm:row-span-1 sm:col-span-2">
            <div class="flex flex-col xs:flex-row gap-4">
              <p class="leading-tight">
                "God has called us to settle in this place and build a community, which is a family of fellowship. He has called us to build a fortress of prayer in this community. He has said to us \"I have chosen this place for My name to live\". The prayer room we build now will be like the Tabernacle, which was a foreshadowing of the temple, and this prayer room is a foreshadowing of the prayer house that we will build."
              </p>
              <p class="leading-tight">
                "We want to cultivate a lifestyle of prayer, and to train a generation who knows how to pray. One of the motivations is 2nd Chronicles 7:14 -- \"if my people, who are called by my name, will humble themselves and pray and seek my face and turn from their wicked ways, then I will hear from heaven, and I will forgive their sin and will heal their land.\" The essence of this vision is encounter God and to hear from Him, that He can reveal what he wants to reveal."
              </p>
            </div>
          </TextBox>
          <TextBox title="Plans">
            <p class="leading-tight">
              "We're building a prayer room in our barn as a first step. We want to invite lots of people to come by, have prayer, and learn prayer. We want to start doing night vigils, and 24/7 prayer. We're going to fill the prayer room with 200 hours of praise and worship running 24/7. We're going to have regular events that you can join if you want to learn how to pray, where we're going to focus on the spiritual gifts, on prophesy, on speaking in tongues, and on soaking in the Spirit."
            </p>
          </TextBox>

          // image
          // <div class="relative min-h-64 col-span-2 border-2 border-black/80 rounded-xl overflow-hidden">
          //   <img class="absolute object-cover top-1/2 -translate-y-1/2 min-w-full" src="/farm.jpeg" />
          // </div>
          <div class="relative flex justify-content-center items-center col-span-2 border-2 border-black/80 rounded-xl overflow-hidden min-h-64">
            <img class="absolute inset-0 min-w-full min-h-full" src="/farm.jpeg" />
          </div>

          <TextBox title="Action" extra_class="row-span-2 xs:row-span-1 xs:col-span-2 md:col-span-3">
            <div class="flex flex-col sm:grid sm:grid-cols-3 gap-6 pb-2 flex-1">
              <div class="flex flex-col gap-2">
                <p class="text-lg font-semibold tracking-tight">"Pray With Us"</p>
                <p class="leading-tight">
                  "Come join us in the prayer room. Pray with us, learn prayer with us, or participate in an event."
                </p>
                <div class="flex-1" />
                <ActionButton text="Pray With Us" href="mailto:contact@prayerhousereitnau.org" />
              </div>
              <div class="flex flex-col gap-2">
                <p class="text-lg font-semibold tracking-tight">"Build With Us"</p>
                <p class="leading-tight">
                  "We need people to help us build and renovate the prayer room. If you have practical skills and time, please come help us."
                </p>
                <div class="flex-1" />
                <ActionButton text="Help Us" href="mailto:contact@prayerhousereitnau.org" />
              </div>
              <div class="flex flex-col gap-2">
                <p class="text-lg font-semibold tracking-tight">"Support Us"</p>
                <p class="leading-tight">
                  "We need fundraising for renovating the prayer room. If you feel led, please give."
                </p>
                <div class="flex-1" />
                <div class="relative text-xs h-12 rounded-lg w-full flex items-center justify-center sm:w-auto border-2 border-black/20 leading-tight">
                  <div class="absolute inset-x-2 -inset-y-2 bg-white" />
                  <p class="absolute inset-0 px-2 sm:px-4">
                    "Give button not available at this time. For now, "
                    <a
                      href="mailto:contact@prayerhousereitnau.org"
                      class="text-cyan-600 hover:text-cyan-500 transition"
                      target="_blank" rel="noreferrer noopener"
                    >
                      "contact us"
                    </a>
                    " to give."
                  </p>
                </div>
              </div>
            </div>
          </TextBox>
        </div>
      </div>
    </div>
  }
}

#[component]
pub fn TextBox(
  title: &'static str,
  #[prop(default = "")] extra_class: &'static str,
  children: Children,
) -> impl IntoView {
  view! {
    <div class={format!("flex flex-col gap-1 p-4 bg-white border-2 border-black/80 rounded-xl shadow-custom {extra_class}")}>
      <p class="pl-1 text-2xl font-semibold tracking-tight">{title}</p>
      {children()}
    </div>
  }
}

#[component]
pub fn ActionButton(text: &'static str, href: &'static str) -> impl IntoView {
  view! {
    <a
      class="bg-cyan-600 hover:bg-cyan-500 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2 focus:ring-offset-slate-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center sm:w-auto mx-2 transition"
      href={href} target="_blank" rel="noreferrer noopener"
    >
      {text}
    </a>
  }
}
