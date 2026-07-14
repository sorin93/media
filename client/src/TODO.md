each history.state has a .tab
to show the tabs menu i use current tab, parent and grandparent tabs
<a href={next url}  [tab={current tab}]>
store.tabs (pathname, parent tab)














// todo: fix redirect same as link.svelte.js

<a href=... tab=...> 
  says change url to ... and tab to ...
  triggers init(), with optional store.url.tab new tab
  if no tab given, it stays the same or if none, it resets to default
  never pass data-tab={tab}




























locations = [
  undefined,
  '/explore#',
  ['/m/{id1}#media_suggestions', '/m/{id1}#media_comments', '/m/{id1}#user_media']
]

location = '/explore#' (Explore tab)   <a href="/m/{id1}" tab="/explore#">  <- need to call init(tab)
Next image   <a href="/m/{id2}" {tab}> keep current tab (explore or parent media etc)



remember current tab must include both pathname and tab  /m/{id1}:media_suggestions OR 

# instead of :
dont consuse tab with pathname+tab. use: location
rename tabs[] to locations[]
rename tab to location
rename store.tabs to store.tabs .set(pathname, location) // pathname has location as parent
both location and locations[] are calculated in init

<a
  href={href}
  data-tab={tab}
  on:init={init}
  use:link
/>

// link.svelte.js

export default function link(node) {
  const click = (e) => {
    e.preventDefault();
    // node.getAttribute('href')
    // node.dataset.tab
    // router logic here...
    node.dispatchEvent(new CustomEvent('init')); // calls init() in the parent
  };

  node.addEventListener('click', click);

  return {
    destroy() {
      node.removeEventListener('click', click);
    }
  };
}


i pass new (href + tab) and i have current (href + tab) from state
_________________________________________________________________


In Svelte specifically

If you're only passing data from the component to the action, another approach is to pass parameters directly to the action:

<a
  href={href}
  use:link={{ tab, init }}
>

export default function link(node, params) {
}


______________________________

Your existing action

You could change:

<a
  href={href}
  data-tab={tab}
  use:link
>

to:

<a
  href={href}
  use:link={{ tab, init }}
>


export default function link(node, params) {

  const click = (e) => {
    // use params.tab here
    if (params.init) params.init(...);
  };

  node.addEventListener('click', click);

  return {
    update(newParams) {
      params = newParams;
    },
    destroy() {
      node.removeEventListener('click', click);
    }
  };
}

__________________________


what A link needs to know:
- current pathname & tab, has them from history.state: { pathname, tab }
- new pathname & tab, gets them from node.getAttribute('href') and node.dataset.tab and sets them in the new state
- i still need to call init() to do 2 things: correct the tab if invalid or missing, 2 fetch the data

every window.history.state has both a { pathname, tab }

<a href="next tab" use:context>
  set map (next tab, current tab (from tab variable))
  set pathname = next tab.split(':')?.[0]
  set tab = next tab
  which calls init() which sets tab = next tab



<a href="" data-trail="next-tab">  <--- but i need to pass both a tab im navigating to, and also which tab is active?
 state.trail[] already contains current page tabs and active one, right?
 if i pass a tab, if it's in the current/parent/grandparent, i bring that to the front, but if i do that, i don't have ancestry of those
 if it's not then i add it to the front?


/m/id		media(id)	i need a tabs (suggested, comments, likes, from user)


Explore
  Media:id1:from_user
    Media:id2:suggested


tab=Media:id1:from_user
id=id1



trail contains 0-3 entries
each entry contains 1-3 items


if it has a trail, it should always have 3 entries, that can be empty [ ]

example of trail crumbs:
  explore
  following
  search:<query>
  user:<UUID>:user_media
  user:<UUID>:comments
  user:<UUID>:likes
  media:<UUID>:suggestions
  media:<UUID>:comments
  media:<UUID>:likes
  media:<UUID>:user_media

active tab is let tab = $state(); // one of the above from trail array, could be market with 0, 1, 2 as well.
let tabs = $state([]); // is calculated based on trail[2] or pathname otherwise


____________________________________________


- user: media, coments, likes
- media: suggestions, comments, likes
- explore:
- following:
- search:


user:id1:media:id2
media:id2:suggestions:id3  => [suggestions, comments, likes]

tab = media:id2:suggestions


which pages can have the same tab?
  user_media (user, media)



explore:mid1
media:mid1:user_media:mid2
user:uid3:user_media:mid4

<a href="..." trail="...">...</a>




window.history.state = {
  trail: ['user:<UUID>:user_media:<UUID>', ...]  // trail:id:tab:id
}

let tab = $state(); // active (can be from trail parent/grandparent too)
let tabs = $state([]);



1. TYPE the pathname states the initial destination

user
media
explore
following
text

2. TAB but then i have multiple choices under each, so it expands:

user_media
user_comments
user_likes

media_suggestions
media_comments
media_likes

explore
following
text



{#if tab === 'explore' || tab === 'following' || tab === '...'}  user_media, media_suggestions, search
  <ListMedia {result} >
{/if}

________________________________________________________________________________________________________________________
users/<UUID>/media        
/users/<UUID>/comments     comments{ user{}, media_id, created_at, type: comment|media, [text], [media{}] }
/users/<UUID>/likes        likes{ user{}, media_id, created_at }

/media/<UUID>/suggestions
/media/<UUID>/comments
/media/<UUID>/likes

/explore
/following
/search/query=<query>
__________
object responses: user{}, users[], media[], texts[], comments[], likes[]
__________
list names:
  user_media
  user_comments
  user_likes

  media_suggestions
  media_comments
  media_likes

  explore
  following
  search







________________________________________________________________________________________________________________________
LOGIC

onaction (onmount or ontab or onmore):
  get context
  get list
  if no list or more fetch
  create or add to list
  set var result

  1. fetch(more = false)
    calculate and retrieve list based on type and tab
    if no list or more = true {
      calculate endpoint
      retrieve data
      set the list eithre creating it or adding to it
    }
    if media_id, find its index

  2. init (on pathname or tab change)
    calculate type, tabs, tab
    called from onmount effect and from Tabs.svelte

  3. 


________________________________________________________________________________________________________________________


history.state = {
  lists: {
    suggested: 'abc',
    user: 'def',
    comments: 'ghi'
  },
  active_tab: 'suggested'
}

Instead of preserving one list:

list_id: 'list' in node.dataset
  ? window.history.state?.list_id
  : undefined,

preserve the entire context:

lists: 'list' in node.dataset
  ? window.history.state?.lists
  : undefined,


window.history.replaceState({
  ...window.history.state,
  lists: {
    ...window.history.state?.lists,
    suggested: suggested_list_id
  }
}, '', window.location.href);


window.history.replaceState({
  ...window.history.state,
  lists: {
    ...window.history.state?.lists,
    user: user_list_id
  }
}, '', window.location.href);


5. Side-pane links

When clicking videos in tabs:
Suggested tab

Use:

data-list="suggested"

const activeTab = history.state.active_tab ?? 'suggested';

const listId =
  history.state.lists?.[activeTab];




use let status = $state.raw() and same for other variables
when showing a media (/m/media_id), that has a list, i need to figure out what does the list belong to: suggested, from user, following, explore etc.
bug: right panel, along with suggested | comments | from user, add 'from following', or 'explore' when the list belongs to those



finish list_id for user, query/text, following, replies, explore etc.
add pagination using offset
.overlay: add top and bottom for prev/next
additional tabs: comments, replies
from user tab: i create a list as well?
what about comments, likes etc?
search bar (_OLD/OLD_Media.svelte)
how to i clear old cache Map entries which accumulate and expire?
post, account, logout
comments
large image overlay: prev, next, user, caption, likes/replies/comments/media count for user
... reset { all: unset; } change to all: initial; or all: revert
dialog: myDialog.showModal();     myDialog.close(); <dialog bind:this={myDialog}>

/explore

/following
/u/<UUID>

/s/<query>

/m/<UUID>



BUG:
- remove store.width, store.height from code, i have it in Textarea.slvete


header: format
more: suggested, users, comments

Bunny.net
Bunny Stream
Storage:        $0.01 per GB per Month per Region
Delivery (CDN): $0.005 per GB
Premium Encodding: $0.02 per Minite
Aspect Ratio: 9:16 1080x1920

Bunny offers a built-in AI system called Automated Content Tagging.
Safety & Moderation: The system automatically scans uploads to detect and tag content. It can identify categories like Adult, Violence, and Spam.
Object/Scene Detection: It can recognize People, Sports, Gaming, and Movies.
A Bunny Webhook can trigger a script that sends the video to an AI model.

Embedding: gemini-embedding-2-preview
To integrate Bunny.net with the gemini-embedding-2-preview model, use Bunny's MP4 Fallback feature to generate direct links that Gemini can use.
Enable MP4 Fallback in your Bunny Library settings before uploading.
Ensure the MP4 file you link to is under 15 MB
Configure your Bunny Video Library to encode a 240p version of every upload
Gemini Embedding 2 processes video at a default of 1 FPS (120 frames total for a 120-second video). You can manually lower the bitrate settings for your low-resolution outputs in the Bunny dashboard to reduce the file size.
Gemini resizes all video to 768 x 768 pixels internally. Providing a 240p video from Bunny.net is ideal because it's already small.


Bunny automatically generates a low-frame-rate Preview Animation for every video. 

    The URL: https://[pull-zone].b-cdn.net/[video-id]/preview.webp
    The Benefit: These are extremely small and already have a very low FPS (often 1–5 FPS depending on video length).
    Compatibility Check: Verify if the specific Gemini implementation accepts .webp as a video source. If it only accepts .mp4, the 240p fallback must be used. 

________________________________________________________________________________________________________________________
in svelte 5 using runes what is the best way to implement a theme (light vs dark)?
using context, or my app's store defined in store.svelte.js or some css stuff, i mean i know the OS itself can send a signal saying it is in light or dark mode etc.?
________________________________________________________________________________________________________________________
large image overlay background for texts
text font size variable
comments
replies
user header
search header
tag header