const fs = require('fs');
const html = fs.readFileSync('error_stealth_dom.html', 'utf8');
const text = html.replace(/<style[^>]*>[\s\S]*?<\/style>/gi, '')
                 .replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '')
                 .replace(/<[^>]+>/g, ' ')
                 .replace(/\s+/g, ' ')
                 .trim();
console.log(text.substring(0, 2000));
