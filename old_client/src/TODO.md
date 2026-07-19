feed container, scroll to window
fixed enlarged, no effects
content 9:16 either vertical or horizontal borders



store.feed[] single feed
each entry has a .type, already comes from the server: "image", "video"
add another .type: "header" so you'll have an entry { type: 'header', url: '...' }

index



2 issues:

1. when navigating back it navigates to the /m/ instead of the context /explore
2. when navigating suggested i run into the 1-2-1-2-1-2 problem


use let status = $state.raw() and same for other variables
add pagination using offset
.overlay: add top and bottom for prev/next
search bar (_OLD/OLD_Media.svelte)
how to i clear old cache Map entries which accumulate and expire?
post, account, logout
comments
large image overlay: prev, next, user, caption, likes/replies/comments/media count for user
... reset { all: unset; } change to all: initial; or all: revert
dialog: myDialog.showModal();     myDialog.close(); <dialog bind:this={myDialog}>

BUG:
- remove store.width, store.height from code, i have it in Textarea.slvete


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
large image overlay background for texts
text font size variable
