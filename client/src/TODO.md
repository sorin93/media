image 'contained'
9:19.5


- make feed multimodal (media grid, tags, users, comments)
- add button to continue feed where you cut it (end of context)
- add titles to header
- load small first in player snippet then big
- large view side not fixed
- filter out duplicates (like from parent context)
- 

- buttons for enlarged media:
  + from user
  + similar images
  + comments, likes, responses combined

- in feed buttons:
  + suggested tags
  + user comments, likes, responses combined



layout has 3 parts:
- small viewport
  + bottom fixed menu bar
  + feed (linked to window and scrollY bar)
  + enlarged media fixed on top of feed
- large viewport
  + top-left fixed menu
  + feed (linked to window and scrollY bar) placed on the right side
  + enlarged media fixed in the middle



.wrapper
  margin-left: var(--nav-width); // based on layout: 0, 66px, 216px
  gap: 0 or 16px
  flex (horizontal)


  MOBILE
  .center-small (flex column, margin auto center, flex 1, max width 768)
    <main>
      <article class="media-small">
        {@render player()}
      </article>
    </main>
    <Tabs {tab} {tabs} />
    <List {test} />
    OR
    <Header {tab} {result} />
    <Tabs {tab} {tabs} />
    <List {test} />
  <Navigation />

  LARGE
  <Navigation />

  <main class="center-media">
    <article class="media-large">
      {@render player()}
    </article>
  </main>
  <aside>
  OR
  <div class="center-large">
    <Header {tab} {result} />
    <Tabs {tab} {tabs} />
    <List {test} />



MOBILE
fixed container taking all viewport
  +in it:
    - [x] top-left
    - vertical menu bar bottom-right
    - caption bottom-left, cut off with more...
    - center play button
  + image: either horizontal or vertical borders, image always full visible

LARGE
container
