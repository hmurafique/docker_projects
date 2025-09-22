FROM ruby:3.2

WORKDIR /app

# Copy Gemfile first to leverage Docker layer caching
COPY Gemfile Gemfile.lock ./

# Install dependencies
RUN bundle install

# Copy app code
COPY . .

EXPOSE 5000
CMD ["bundle", "exec", "ruby", "app.rb", "-o", "0.0.0.0", "-p", "5000"]
