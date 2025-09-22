FROM ruby:3.2
WORKDIR /app
COPY . .
RUN bundle install
EXPOSE 5000
CMD ["ruby", "app.rb", "-o", "0.0.0.0", "-p", "5000"]
