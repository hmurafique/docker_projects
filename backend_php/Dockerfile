FROM php:8.2-cli
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y unzip git \
    && curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer \
    && composer install
EXPOSE 5000
CMD ["php", "-S", "0.0.0.0:5000", "index.php"]
