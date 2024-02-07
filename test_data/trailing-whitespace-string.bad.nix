{
  foo = ''
# And finally deny all other access to this proxy
http_access deny all
       
# Squid normally listens to port 3128
http_port 

# Set outgoing address to service ip 
tcp_outgoing_address         
        
# Leave coredumps in the first cache dir
coredump_dir /var/cache/squid
'';
}
