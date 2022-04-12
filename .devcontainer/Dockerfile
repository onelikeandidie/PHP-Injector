FROM ubuntu

USER root
RUN passwd -d root

# Install MariaDB client
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y \ 
    neovim nano \
    mariadb-client sudo curl git \
    apache2 php7.4-dev libapache2-mod-php7.4 \
    # php7.4-mysql php7.4-curl php7.4-gd php7.4-intl php-pear php-imagick php7.4-imap php-memcache php-mbstring \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# ZSH
RUN ["apt-get", "update"]
RUN ["apt-get", "install", "-y", "zsh"]
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN chsh -s $(which zsh)

# Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN sed -E -i -e '/Directory \/var\/www/,/\/Directory/s/AllowOverride None/AllowOverride All/' /etc/apache2/apache2.conf
RUN sed -E -i -e 's/DirectoryIndex (.*)$/DirectoryIndex index.php \1/g' /etc/apache2/apache2.conf
RUN sed -E -i -e 's/User _www/User root/g' /etc/apache2/apache2.conf
RUN sed -E -i -e 's/Group _www/User root/g' /etc/apache2/apache2.conf
RUN sed -E -i -e 's/Directory \/var\/www/Directory \/workspace/g' /etc/apache2/apache2.conf
# Change the default apache website to the root of the project
RUN sed -E -i -e 's/AllowOverride None/AllowOverride All/g' /etc/apache2/sites-available/000-default.conf
RUN sed -E -i -e 's/DocumentRoot \/var\/www\/html/DocumentRoot \/workspace/g' /etc/apache2/sites-available/000-default.conf
RUN sed -E -i -e 's/#ServerName www.example.com/ServerName 127.0.0.1/g' /etc/apache2/sites-available/000-default.conf
RUN sed -E -i -e 's/VirtualHost \*:80/VirtualHost \*:8080/g' /etc/apache2/sites-available/000-default.conf
RUN sed -E -i -e 's/<DocumentRoot \/workspace>/&\nAccessFileName ".htaccess"/g' /etc/apache2/sites-available/000-default.conf
RUN sed -E -i -e 's/Listen 80/Listen 8080/g' /etc/apache2/ports.conf
# Add support for .htaccess files
RUN a2enmod rewrite

# Enable xdebug
RUN pecl install xdebug

RUN echo "# xdebug stuff start" >> /etc/php/7.4/apache2/php.ini  \
 && echo "[xdebug]" >> /etc/php/7.4/apache2/php.ini  \
 && echo "zend_extension = xdebug.so" >> /etc/php/7.4/apache2/php.ini  \
#&& echo "zend_extension = /opt/php/lib/modules/xdebug.so" >> /etc/php/7.4/apache2/php.ini  \
 && echo "xdebug.mode = develop,trace,debug,profile" >> /etc/php/7.4/apache2/php.ini  \
 && echo "xdebug.client_host = localhost" >> /etc/php/7.4/apache2/php.ini  \
 && echo "xdebug.client_port = 9003" >> /etc/php/7.4/apache2/php.ini  \
 && echo "xdebug.start_with_request = yes" >> /etc/php/7.4/apache2/php.ini  \
#&& echo "xdebug.remote_port = 9003" >> /etc/php/7.4/apache2/php.ini \
#&& echo "xdebug.remote_enable = 1" >> /etc/php/7.4/apache2/php.ini \
#&& echo "xdebug.default_enable = 0" >> /etc/php/7.4/apache2/php.ini \
#&& echo "xdebug.profiler_enable = 1" >> /etc/php/7.4/apache2/php.ini \
#&& echo "xdebug.auto_trace = 1" >> /etc/php/7.4/apache2/php.ini \
#&& echo "xdebug.coverage_enable = 1" >> /etc/php/7.4/apache2/php.ini \
 && echo "# xdebug stuff finish" >> /etc/php/7.4/apache2/php.ini

# Enable assertions for debugging
RUN sed -E -i -e 's/zend.assertions = -1/zend.assertions = 1/g' /etc/php/7.4/apache2/php.ini 

RUN echo 'export XDEBUG_SESSION=1'

# Start apache and sleep forever
#EXPOSE 8080
#CMD /bin/bash source /etc/apache2/envvars && apache2 -D FOREGROUND && sleep infinity