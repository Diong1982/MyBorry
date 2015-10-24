# Block until the message is received
receive do
  {:msg, contents} -> IO.puts contents
end
