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

  Cloudflare > Compute > Workers & Pages > [Worker] > Bindings > Add Binding > Workers AI > AI > MY_GATEWAY > google-cloud.json

  Cloudflare > Workers & Pages > [Worker] > ... I think Settings, to add add path for worker like sorin.cc/media/new/upload
*/

// Bindings: AI, BUCKET, IMAGES

const transform = async (env, bytes, width, height, format, quality) => env.IMAGES
  .input(bytes)
  .transform({ width, height, fit: 'cover', metadata: 'none' })
  .output({ format, quality });

const upload = async (env, bytes, width, height, format, quality, name) => {
  const result = await transform(env, bytes, width, height, format, quality);
  const response = result.response();
  await env.BUCKET.put(name, response.body, {
    httpMetadata: { contentType: result.contentType() }
  });
};

const fetchApi = async (path, method, authorization, body) => {
  try {
    const res = await fetch(`https://api.sorin.cc${path}`, {
      method,
      headers: {
          'Authorization': authorization,
          'Content-Type':  'application/json',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(await res.text());
    return await res.json();
  } catch (err) {
    throw new Error(err);
  }
};

const analyze = async (env, bytes) => {
  const result = await transform(env, bytes, 135, 240, 'image/jpeg', 80);
  bytes = new Uint8Array(await result.response().arrayBuffer());
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
          image: bytes
        }
      ]
    }];
    const aiResult = await env.AI.run('@cf/meta/llama-3.2-11b-vision-instruct', {
      image: [...bytes], // todo: remove
      messages,
    });
    console.log(aiResult);
};

const getEmbedding = async (env, bytes) => {
  const result = await transform(env, bytes, 512, 512, 'image/jpeg', 80);
  const arrayBuffer = await result.response().arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  let binaryString = '';
  for (let i = 0; i < uint8Array.byteLength; i += 8192) {
    binaryString += String.fromCharCode.apply(null, uint8Array.subarray(i, i + 8192));
  }
  const bytesBase64Encoded = btoa(binaryString);
  const payload = {
    content: {
      parts: [ { inlineData: { mimeType: 'image/jpeg', data: bytesBase64Encoded } } ]
    },
    outputDimensionality: 3072,
  };
  const GOOGLE_CLOUD_MODEL = 'gemini-embedding-2-preview'; 
  const GOOGLE_CLOUD_PROJECT_ID = 'ringed-prism-468006-n2'; 
  const GOOGLE_CLOUD_REGION = 'us-central1'; 
  const CLOUDFLARE_ACCOUNT_ID = 'c059f5adb15a5770b03519161de7b0f6'; 
  const CLOUDFLARE_GATEWAY_ID = 'my-gateway'; 
  const CLOUDFLARE_GATEWAY_TOKEN = 'GpAYZIH30oyBZwcqW_B9mfKB-yCnR3FxABAiWQw2';
  const CLOUDFLARE_GATEWAY_URL = `https://gateway.ai.cloudflare.com/v1/${CLOUDFLARE_ACCOUNT_ID}/${CLOUDFLARE_GATEWAY_ID}/google-vertex-ai/v1/projects/${GOOGLE_CLOUD_PROJECT_ID}/locations/${GOOGLE_CLOUD_REGION}/publishers/google/models/${GOOGLE_CLOUD_MODEL}:embedContent`;
  let res = await fetch(CLOUDFLARE_GATEWAY_URL, {
    method: 'POST',
    headers: {
      'Authorization': CLOUDFLARE_GATEWAY_TOKEN,
      'Content-Type':  'application/json',
    },
    body: JSON.stringify(payload),
  });
  if (!res.ok) throw new Error(await res.text());
  res = await res.json();
  const embedding = res.embedding?.values;
  if (!embedding) throw new Error('No embedding returned');
  return embedding; 
};

const getBlurhash = async (env, bytes) => {
  /*
  const result = await transform(env, bytes, 135, 240, 'image/png', 80);
  bytes = new Uint8Array(await result.response().arrayBuffer());
  const base64Image = btoa(String.fromCharCode(...bytes));
  */
  const blurhash = null;
  return blurhash;
};

export default {
  async fetch(request, env, _ctx) {
    const authorization = request.headers.get('Authorization')
    const headers = {
      'Access-Control-Allow-Origin': 'http://localhost:8080',
      'Access-Control-Allow-Credentials': 'true',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Authorization, Origin, X-Requested-With, Content-Type, Accept',
      'Access-Control-Max-Age': '86400',
    };
    if (request.method === 'OPTIONS') {
      return new Response(null, { status: 204, headers });
    } else if (request.method !== 'POST') return new Response('Method not allowed', { status: 405, headers });

    try {
      const formData = await request.formData();
      const parent_media_id = formData.get('parent_media_id') || null;
      const caption = formData.get('caption') || null;
      const file = formData.get('file');
      const type = file?.type?.slice(0, 5);
      if (parent_media_id && !/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(parent_media_id)) throw new Error('Invalid parent media');
      if (caption && (caption.length < 2 || caption.length > 100)) throw new Error('Invalid caption');
      if (!(file instanceof File)) throw new Error('Missing media');
      if (type !== 'image') throw new Error('Invalid media type');
      if (file.size < 51_200 || file.size > 10_485_760) throw new Error('Invalid media size');
      const bytes = await file.arrayBuffer();
      if (bytes.byteLength < 51_200 || bytes.byteLength > 10_485_760) throw new Error('Invalid media size');

      // Analyze for safery and people
      await analyze(env, bytes);

      // Create media record
      const { media: { media_id }}  = await fetchApi('/media', 'POST', authorization, { parent_media_id, type, caption });

      // Create media variants
      const [embedding, blurhash, , ,] = await Promise.all([
        await getEmbedding(env, bytes),
        await getBlurhash(env, bytes),
        upload(env, bytes, 100,  100, 'image/avif', 50, `${media_id}/small`),
        upload(env, bytes, 240,  320, 'image/avif', 65, `${media_id}/medium`),
        upload(env, bytes, 675, 1200, 'image/avif', 80, `${media_id}/large`),
      ]);

      // Activate media
      const media = await fetchApi(`/media/${media_id}`, 'POST', authorization, { media_id, embedding, blurhash });

      return new Response(JSON.stringify(media), { status: 200, headers });
    } catch (err) {
        return new Response(err?.message || 'Error', { status: 500, headers });
    }
  },
};