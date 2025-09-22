
require 'sinatra'
require 'json'

get '/' do
  content_type :json
  { msg: "Hello from Ruby backend 💎" }.to_json
end
