OLD

/*
  Google Cloud > [Project] > Vertex AI >
  Google Cloud > [Project] > IAM & Admin > IAM > Allow > [Service Account] > Vertex AI Service Agent (for Cloudflare)
                                                                             Vertex AI User Agent (for Rust)


  Compute > Workers & Pages > [Worker] > AI > AI Gateways > [my-gateway] > Settings > Authenticated Gateway > ON > Create Authentication Token > heders: { 'Authorization': '[token]', }
  Compute > Workers & Pages > [Worker] > AI > AI Gateways > [my-gateway] > Provider Keys > Google Vertex AI > [google-cloud.json], us-central1, default

  Compute > Workers & Pages > [Worker] > Edit Code (use Chromium Browser, can also call the worker url here)
  Compute > Workers & Pages > [Worker] > Settings > Variables and Secrets > Add >

  Compute > Workers & Pages > [Worker] > Bindings > Add Binding > R2 Bucket > BUCKET, media
  Compute > Workers & Pages > [Worker] > Bindings > Add Binding > Images > IMAGES
  Compute > Workers & Pages > [Worker] > Bindings > Add Binding > Workers AI > AI

  Cloudflare > R2 Bucket > Settings > Event Notifications > Todo later for original folder in the r2 bucket

  Cloudflare > R2 Bucket > Settings > CORS Policy:
    [
        {
        "AllowedOrigins": ["*"],
        "AllowedMethods": ["PUT", "GET", "HEAD"],
        "AllowedHeaders": ["*"],
        "ExposeHeaders": ["ETag"],
        "MaxAgeSeconds": 3600
        }
    ]

  Cloudflare > Networking > Tunnels > make sure you delete previous existent dns entries for subdomain before creating the tunnel

  Compute > Workers & Pages > [Worker] > Bindings > Add Binding > Workers AI > AI > MY_GATEWAY > google-cloud.json



  Url: https://{worker_id}.1y.workers.dev/?access_key={access_key}&media_id={media_id}
  https://media-353.1y.workers.dev?media_id=d8bb160e-e472-4572-8703-53c6f73985ca
*/

export default {
  async fetch(request, env) {
    const headers = {
      'Access-Control-Allow-Origin': '*',
    };

    const url = new URL(request.url);
    const media_id = url.searchParams.get('media_id');
    if (media_id?.length !== 36) return new Response('Invalid media_id', { status: 422, headers });
    console.log(`media_id=${media_id}`);

    const originalKey = `${media_id}/original`;
    const object = await env.BUCKET.get(originalKey);
    if (!object) return new Response('Image not found', { status: 404, headers });
    const originalBlob = await object.blob();

    // Create additional images
    const variants = [
      { name: 'small',     format: 'image/avif', width: 100,  height:  100, quality: 50 },
      { name: 'medium',    format: 'image/avif', width: 240,  height:  320, quality: 65 },
      { name: 'large',     format: 'image/avif', width: 675,  height: 1200, quality: 80 },
      { name: 'embedding', format: 'image/jpeg', width: 512,  height:  512, quality: 80 }, // square aspect ratio
      { name: 'analyze',   format: 'image/jpeg', width: 135,  height:  240, quality: 80 }, // same aspect-ratio as 'large'
      { name: 'blurhash', format: 'rgba', width: 28, height: 49 },
    ];

    /*

      todo1: i dont need to save to r2: embedding, analyze, blurhash
      todo2: blurhash needs rgba
      todo3: generate analyze. only if ok generate the rest


    */

    try {
      await Promise.all(variants.map(async (v) => {
        const result = await env.IMAGES
          .input(originalBlob)
          .transform({
            width: v.width,
            height: v.height,
            fit: 'cover',
            metadata: 'none',
          })
          .output({
            format:  v.format,
            quality: v.quality,
          });

        const response = result.response();
        await env.BUCKET.put(`${media_id}/${v.name}`, response.body, {
          httpMetadata: { contentType: result.contentType() }
        });
      }));

      // Analyze the 'analyze' variant image with llama-3.2-11b-vision-instruct

      // First run only (comment afterwards)
      // await env.AI.run('@cf/meta/llama-3.2-11b-vision-instruct', { prompt: 'agree' });

      const analyzeKey = `${media_id}/analyze`;
      const analyzeObject = await env.BUCKET.get(analyzeKey);
      if (!analyzeObject) return new Response('Analyze image not found', { status: 500, headers });
      const analyzeBytes = new Uint8Array(await analyzeObject.arrayBuffer());

      const messages = [
        {
          role: 'system',
          content: 'Analyze the image and respond with one single word: ok, unsafe, or no-person.'
        },
        {
          role: 'user',
          content: [
            {
              type: 'text',
              text: `Is the image safe and contains at least person?
- safe=true only if this is a normal real-photo image and NOT NSFW, NOT sexual, NOT spam, NOT advertising, NOT heavy-text, NOT cartoon/anime/illustration, NOT obvious AI-generated, and NOT non-photographic.
- person=true only if there is at least one clearly visible real human person.
Answer with one word`,
            },
            {
              type: 'image',
              image: analyzeBytes
            }
          ]
        }
      ];

      const aiResult = await env.AI.run('@cf/meta/llama-3.2-11b-vision-instruct', {
        image: [...analyzeBytes],
        messages,
      });

      console.log(aiResult);

      /*
      // Generate image embedding vector with Google Vertex multimodalembedding@001 model, using the Cloudflare Gateway
      const object = await env.BUCKET.get(`${media_id}/embedding`);
      if (!object) return new Response('Embedding image not found', { status: 500, headers });

      // 2. Convert R2 object body to Base64
      // Vertex AI requires the image bytes to be base64 encoded strings
      const arrayBuffer = await object.arrayBuffer();
      const base64Image = btoa(String.fromCharCode(...new Uint8Array(arrayBuffer)));

      // 3.<!--citation:1--> Prepare Vertex AI Payload
      // Model: multimodalembedding@001
      const payload = {
        instances: [
          {
            image: {
              bytesBase64Encoded: base64Image
            }
          }
        ],
        // Optional: limit dimensions if needed (e.g., 1408, 512, 256, 128)
        // parameters: { dimension: 1408 } 
      };

      // 4. Send to Cloudflare AI Gateway
      // Replace {account_id} and {gateway_id} with your actual values
      // The endpoint format is: 
      // https://gateway.ai.cloudflare.com/v1/{account_id}/{gateway_id}/google-vertex-ai/v1/projects/{project_id}/locations/{region}/publishers/google/models/{model}:predict

           
      const ACCOUNT_ID = 'c059f5adb15a5770b03519161de7b0f6'; //cloudflare
      const GATEWAY_ID = 'my-gateway'; // Your AI Gateway name
      const PROJECT_ID = 'ringed-prism-468006-n2'; // Your Google Cloud Project ID
      const REGION = 'us-central1'; // Your Google Cloud Region
      const MODEL = 'multimodalembedding@001';

      const gatewayUrl = `https://gateway.ai.cloudflare.com/v1/${ACCOUNT_ID}/${GATEWAY_ID}/google-vertex-ai/v1/projects/${PROJECT_ID}/locations/${REGION}/publishers/google/models/${MODEL}:predict`;
      //                  https://gateway.ai.cloudflare.com/v1/${ACCOUNT_ID}/${GATEWAY_ID}/google-vertex-ai
      // *read this https://developers.cloudflare.com/ai-gateway/usage/providers/vertex/


      const aiResponse = await fetch(gatewayUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload),
      });

      console.log('Gateway response:', aiResponse);

      if (!aiResponse.ok) {
        const errorText = await aiResponse.text();
        return new Response(`Vertex AI Error: ${errorText}`, { status: aiResponse.status });
      }

      const aiData = await aiResponse.json();

      // 5. Extract Embedding
      // The response structure is { predictions: [ { imageEmbedding: [...] } ] }
      const embedding = aiData.predictions?.[0]?.imageEmbedding;
      */

      // Activate media
      console.log(`ACTIVATING  https://api.sorin.cc/media/${media_id}`);
      let res = await fetch(`https://api.sorin.cc/media/${media_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          embedding: Array(512).fill(0.0),
          blurhash: 'mock-blurhash',
        }),
      });
      const json = await res.json();
      console.log(json);

      // Delete temporary files
      await env.BUCKET.delete(`${media_id}/original`);
      await env.BUCKET.delete(`${media_id}/embedding`);
      await env.BUCKET.delete(`${media_id}/analyze`);


      return new Response('OK', { status: 200, headers });
    } catch (error) {
      return new Response(error, { status: 403, headers });
    }
  }
};


//______________________________________________________________________________________________________________________

/*

package.json
{
  "name": "your-worker-name",
  "type": "module",
  "dependencies": {
    "blurhash": "^2.0.5"
  }
}

import { encode } from "blurhash";

if (v.name === 'blurhash') {
    // 1. Get raw bytes (Width * Height * 4)
    const rawPixels = await result.arrayBuffer(); 
    
    // 2. Encode directly to string (using 3x5 components for 9:16)
    generatedBlurhash = encode(
      new Uint8ClampedArray(rawPixels), 
      v.width, 
      v.height, 
      3, 5
    );
    
    // Do NOT put this one in R2; we just wanted the string!
    return; 
  }

  */