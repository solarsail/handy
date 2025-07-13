FROM adas-img.nioint.com/aa-platform/nginx:1.25.2

COPY dist/* /usr/share/nginx/html
