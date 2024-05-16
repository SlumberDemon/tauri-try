export async function load() {
    const response = await fetch("http://localhost:8008/api", {
      method: "GET",
    });
  
    const data = await response.json();
  
    return {
      data: data,
    };
  }
  