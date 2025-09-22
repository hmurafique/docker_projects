package com.example.demo.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.Map;

@RestController
public class HelloController {
    @GetMapping("/")
    public Map<String, String> home() {
        return Map.of("msg", "Hello from Java Spring Boot backend ☕");
    }
}
