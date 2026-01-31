// k6 benchmark script for Doublets GraphQL
import { GraphQLBenchmark, queries, randomVariables } from './common.js';
import { group, sleep } from 'k6';

export let options = {
  scenarios: {
    create_load: {
      executor: 'constant-arrival-rate',
      rate: 100, // 100 requests per second
      timeUnit: '1s',
      duration: '30s',
      preAllocatedVUs: 10,
      maxVUs: 50,
      exec: 'createScenario',
    },
    update_load: {
      executor: 'constant-arrival-rate',
      rate: 100,
      timeUnit: '1s',
      duration: '30s',
      preAllocatedVUs: 10,
      maxVUs: 50,
      exec: 'updateScenario',
      startTime: '35s',
    },
    delete_load: {
      executor: 'constant-arrival-rate',
      rate: 100,
      timeUnit: '1s',
      duration: '30s',
      preAllocatedVUs: 10,
      maxVUs: 50,
      exec: 'deleteScenario',
      startTime: '70s',
    },
    read_load: {
      executor: 'constant-arrival-rate',
      rate: 200,
      timeUnit: '1s',
      duration: '60s',
      preAllocatedVUs: 20,
      maxVUs: 100,
      exec: 'readScenario',
      startTime: '105s',
    }
  },
  thresholds: {
    http_req_duration: ['p(95)<1000'], // 95% of requests should be below 1s
    http_req_failed: ['rate<0.1'], // Error rate should be below 10%
  },
};

const doublets = new GraphQLBenchmark('http://localhost:60341/v1/graphql');

export function createScenario() {
  group('Create Operations', function() {
    // Create point link
    doublets.query(queries.createPointLink);
    
    // Create regular link
    const vars = randomVariables();
    doublets.query(queries.createLink, { 
      source: vars.source, 
      target: vars.target 
    });
  });
}

export function updateScenario() {
  group('Update Operations', function() {
    const vars = randomVariables();
    doublets.query(queries.updateLink, {
      id: vars.id,
      source: vars.source,
      target: vars.target
    });
  });
}

export function deleteScenario() {
  group('Delete Operations', function() {
    const vars = randomVariables();
    doublets.query(queries.deleteLink, { id: vars.id });
  });
}

export function readScenario() {
  group('Read Operations', function() {
    const vars = randomVariables();
    
    // Each All
    doublets.query(queries.allLinks, { limit: 10, offset: 0 });
    
    // Each Identity  
    doublets.query(queries.linkById, { id: vars.id });
    
    // Each Concrete
    doublets.query(queries.concreteLinks, {
      source: vars.source,
      target: vars.target,
      limit: 10,
      offset: 0
    });
    
    // Each Outgoing
    doublets.query(queries.outgoingLinks, {
      source: vars.source,
      limit: 10,
      offset: 0
    });
    
    // Each Incoming
    doublets.query(queries.incomingLinks, {
      target: vars.target,
      limit: 10,
      offset: 0
    });
  });
}