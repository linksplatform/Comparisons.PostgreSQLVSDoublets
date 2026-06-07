// Common utilities for GraphQL benchmarking with k6
import http from 'k6/http';
import { check } from 'k6';

export class GraphQLBenchmark {
  constructor(endpoint, headers = {}) {
    this.endpoint = endpoint;
    this.headers = {
      'Content-Type': 'application/json',
      ...headers
    };
  }

  query(query, variables = {}) {
    const payload = JSON.stringify({
      query: query,
      variables: variables
    });
    
    const response = http.post(this.endpoint, payload, { headers: this.headers });
    
    check(response, {
      'GraphQL request successful': (r) => r.status === 200,
      'No GraphQL errors': (r) => {
        const body = JSON.parse(r.body);
        return !body.errors;
      }
    });
    
    return response;
  }
}

// GraphQL queries and mutations
export const queries = {
  // Create operations
  createPointLink: `
    mutation CreatePointLink {
      createPointLink {
        id
        source
        target
      }
    }
  `,
  
  createLink: `
    mutation CreateLink($source: ID!, $target: ID!) {
      createLink(input: { source: $source, target: $target }) {
        id
        source
        target
      }
    }
  `,
  
  // Update operation
  updateLink: `
    mutation UpdateLink($id: ID!, $source: ID, $target: ID) {
      updateLink(input: { id: $id, source: $source, target: $target }) {
        id
        source
        target
      }
    }
  `,
  
  // Delete operation
  deleteLink: `
    mutation DeleteLink($id: ID!) {
      deleteLink(id: $id)
    }
  `,
  
  // Read operations - Each variants
  allLinks: `
    query AllLinks($limit: Int, $offset: Int) {
      allLinks(limit: $limit, offset: $offset) {
        id
        source
        target
      }
    }
  `,
  
  linkById: `
    query LinkById($id: ID!) {
      linkById(id: $id) {
        id
        source
        target
      }
    }
  `,
  
  concreteLinks: `
    query ConcreteLinks($source: ID!, $target: ID!, $limit: Int, $offset: Int) {
      concreteLinks(source: $source, target: $target, limit: $limit, offset: $offset) {
        id
        source
        target
      }
    }
  `,
  
  outgoingLinks: `
    query OutgoingLinks($source: ID!, $limit: Int, $offset: Int) {
      outgoingLinks(source: $source, limit: $limit, offset: $offset) {
        id
        source
        target
      }
    }
  `,
  
  incomingLinks: `
    query IncomingLinks($target: ID!, $limit: Int, $offset: Int) {
      incomingLinks(target: $target, limit: $limit, offset: $offset) {
        id
        source
        target
      }
    }
  `
};

// Generate random variables for testing
export function randomVariables() {
  const id = Math.floor(Math.random() * 1000) + 1;
  return {
    id: id.toString(),
    source: (Math.floor(Math.random() * 1000) + 1).toString(),
    target: (Math.floor(Math.random() * 1000) + 1).toString(),
    limit: 10,
    offset: 0
  };
}