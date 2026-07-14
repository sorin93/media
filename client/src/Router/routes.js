// Auth
import Login from '../User/Login.svelte';
import Logout from '../User/Logout.svelte';

// User
import Register from '../User/Register.svelte';
import User from '../User/User.svelte';

// Media
import Media from '../Media/Media.svelte';
import NewMedia from '../Media/NewMedia.svelte';

// General
import Home from '../Home.svelte';
import Help from '../Help.svelte';
import Privacy from '../Privacy.svelte';
import Terms from '../Terms.svelte';

import { UUID } from '../lib/regexp';

const mediaRegex = new RegExp(`^/m/${UUID}$`);
const textRegex = /^\/s(\/[^/]{0,100})?$/;
const userRegex = new RegExp(`^/u/${UUID}$`);

/*
  0 Name
  1 Svg
  2 Component
  3 Requires login (true, false, null <either way>)
  4 Show in rail
  5 path 1: match to find component
  6 [path 2]: link to redirect (only for ones showing in rail)
  7 [path 3]: match to activate in rail (only for ones showing in rail)
*/

export default [
  // Name        Svg             Component       Auth   Menu   Component             Link         Active
  [ 'Home',      'home',          Home,          null,  true,  '/',                  '/',         '/'                  ],
  [ 'Login',     'login',         Login,         false, true,  '/login',             '/login',    '/login'             ],
  [ 'Register',  'person',        Register,      false, true,  '/register',          '/register', '/register'          ],
  [ 'User',      'person',        User,          true,  true,  /^\/users(?:\/.*)?$/, '/users',     /^\/users(?:\/.*)?$/],

  [ 'Media',     'play_circle',   Media,         null,  true, '/explore',           '/media',     '/explore'         ],
  [ 'Media',     'play_circle',   Media,         null,  true, '/following',          '/media',     '/following'        ],
  [ 'Media',     'play_circle',   Media,         null,  true, mediaRegex,            '/media',     mediaRegex          ],
  [ 'Media',     'play_circle',   Media,         null,  true, textRegex,             '/media',     textRegex           ],
  [ 'Media',     'play_circle',   Media,         null,  true, userRegex,             '/media',     userRegex           ],

  [ 'New Media', 'play_circle',   NewMedia,      true,  true, '/media/new',          '/media/new', '/media/new'        ],

  [ 'Help',      'help',          Help,          null,  true,  '/help',              '/help',     '/help'              ],
  [ 'Logout',    'logout',        Logout,        true,  true,  '/logout',            '/logout',   '/logout'            ],
  [ 'Terms',     'info',          Terms,         null,  false, '/terms'                                                ],
  [ 'Privacy',   'info',          Privacy,       null,  false, '/privacy'                                              ],
];