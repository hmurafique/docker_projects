<?php
require __DIR__ . '/vendor/autoload.php';

$app = new \Slim\App();

$app->get('/', function ($request, $response, $args) {
    return $response->withJson(['msg' => 'Hello from PHP backend 🐘']);
});

$app->run();
