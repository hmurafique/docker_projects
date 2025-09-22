FROM maven:3.9.5-eclipse-temurin-17 AS builder
WORKDIR /app
COPY . .
RUN mvn clean package -DskipTests

FROM eclipse-temurin:17
WORKDIR /app
COPY --from=builder /app/target/demo-1.0.0.jar app.jar
EXPOSE 5000
CMD ["java", "-jar", "app.jar"]
