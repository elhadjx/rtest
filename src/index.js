const ping = require('ping');

const hosts = ['192.168.31.1',
    '192.168.31.15',
    '192.168.31.23',
    '192.168.31.68',
    '192.168.31.107',
    '192.168.31.123',
    '192.168.31.178',
    '192.168.31.202',
];

hosts.forEach(function (host) {
    ping.sys.probe(host, function (isAlive) {
        var msg = isAlive ? 'host ' + host + ' is alive' : 'host ' + host + ' is dead';
        console.log(msg);
    });
});
