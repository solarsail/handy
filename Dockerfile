FROM adas-img.nioint.com/aa-platform/nginx:1.25.2

COPY dist/* /usr/share/nginx/html
COPY assets/icon-256.png /usr/share/nginx/html/assets/icon-256.png
