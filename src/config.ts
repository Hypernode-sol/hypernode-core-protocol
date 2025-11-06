/**
 * Configuration management for Hypernode Core Protocol
 * Centralized configuration for all parameters and constants
 */

import dotenv from 'dotenv';

dotenv.config();

/**
 * Protocol configuration interface
 */
export interface ProtocolConfig {
  // Network configuration
  network: 'devnet' | 'testnet' | 'mainnet-beta';
  solanaRpcUrl: string;
  solanaCommitment: 'processed' | 'confirmed' | 'finalized';

  // Program IDs
  programIds: {
    nodes: string;
    jobs: string;
    staking: string;
    rewards: string;
  };

  // Staking configuration
  staking: {
    minStakeAmount: number;
    maxStakeAmount: number;
    minDurationSeconds: number;
    maxDurationSeconds: number;
    baseMultiplier: number;
    maxMultiplier: number;
  };

  // Job configuration
  jobs: {
    minBudget: number;
    maxBudget: number;
    jobTimeoutSeconds: number;
    maxRetriesPerJob: number;
    escrowFeePercentage: number;
  };

  // Node configuration
  nodes: {
    minReputation: number;
    maxConcurrentJobs: number;
    slashingPercentage: number;
    slashingCooldownSeconds: number;
  };

  // Rewards configuration
  rewards: {
    reflectionPercentage: number;
    dailyRewardRate: number;
    compoundingEnabled: boolean;
  };

  // Feature flags
  features: {
    ipfsIntegration: boolean;
    dynamicQueue: boolean;
    tokenReflection: boolean;
    autoSlashing: boolean;
  };

  // Security configuration
  security: {
    requireSignerVerification: boolean;
    maxTransactionSize: number;
    rateLimit: number;
  };
}

/**
 * Load and validate configuration from environment variables
 */
export function loadConfig(): ProtocolConfig {
  const network = (process.env.SOLANA_NETWORK || 'mainnet-beta') as 'devnet' | 'testnet' | 'mainnet-beta';

  // Validate required environment variables
  const requiredVars = [
    'SOLANA_RPC_URL',
    'HYPERNODE_NODES_PROGRAM_ID',
    'HYPERNODE_JOBS_PROGRAM_ID',
    'HYPERNODE_STAKING_PROGRAM_ID',
    'HYPERNODE_REWARDS_PROGRAM_ID',
  ];

  const missingVars = requiredVars.filter((v) => !process.env[v]);
  if (missingVars.length > 0) {
    throw new Error(`Missing required environment variables: ${missingVars.join(', ')}`);
  }

  const config: ProtocolConfig = {
    // Network configuration
    network,
    solanaRpcUrl: process.env.SOLANA_RPC_URL!,
    solanaCommitment: (process.env.SOLANA_COMMITMENT || 'confirmed') as any,

    // Program IDs
    programIds: {
      nodes: process.env.HYPERNODE_NODES_PROGRAM_ID!,
      jobs: process.env.HYPERNODE_JOBS_PROGRAM_ID!,
      staking: process.env.HYPERNODE_STAKING_PROGRAM_ID!,
      rewards: process.env.HYPERNODE_REWARDS_PROGRAM_ID!,
    },

    // Staking configuration
    staking: {
      minStakeAmount: parseInt(process.env.MIN_STAKE_AMOUNT || '100000000', 10),
      maxStakeAmount: parseInt(process.env.MAX_STAKE_AMOUNT || '1000000000000000', 10),
      minDurationSeconds: parseInt(process.env.MIN_STAKE_DURATION || String(14 * 24 * 60 * 60), 10),
      maxDurationSeconds: parseInt(process.env.MAX_STAKE_DURATION || String(4 * 365 * 24 * 60 * 60), 10),
      baseMultiplier: parseInt(process.env.BASE_MULTIPLIER || '1000', 10),
      maxMultiplier: parseInt(process.env.MAX_MULTIPLIER || '4000', 10),
    },

    // Job configuration
    jobs: {
      minBudget: parseInt(process.env.MIN_JOB_BUDGET || '1000000', 10),
      maxBudget: parseInt(process.env.MAX_JOB_BUDGET || '1000000000000', 10),
      jobTimeoutSeconds: parseInt(process.env.JOB_TIMEOUT_SECONDS || '3600', 10),
      maxRetriesPerJob: parseInt(process.env.MAX_RETRIES || '3', 10),
      escrowFeePercentage: parseFloat(process.env.ESCROW_FEE || '1.0'),
    },

    // Node configuration
    nodes: {
      minReputation: parseInt(process.env.MIN_REPUTATION || '100', 10),
      maxConcurrentJobs: parseInt(process.env.MAX_CONCURRENT_JOBS || '50', 10),
      slashingPercentage: parseFloat(process.env.SLASHING_PERCENTAGE || '10.0'),
      slashingCooldownSeconds: parseInt(process.env.SLASHING_COOLDOWN || String(24 * 60 * 60), 10),
    },

    // Rewards configuration
    rewards: {
      reflectionPercentage: parseFloat(process.env.REFLECTION_PERCENTAGE || '2.0'),
      dailyRewardRate: parseFloat(process.env.DAILY_REWARD_RATE || '0.1'),
      compoundingEnabled: process.env.COMPOUNDING_ENABLED !== 'false',
    },

    // Feature flags
    features: {
      ipfsIntegration: process.env.IPFS_INTEGRATION !== 'false',
      dynamicQueue: process.env.DYNAMIC_QUEUE !== 'false',
      tokenReflection: process.env.TOKEN_REFLECTION !== 'false',
      autoSlashing: process.env.AUTO_SLASHING !== 'false',
    },

    // Security configuration
    security: {
      requireSignerVerification: process.env.REQUIRE_SIGNER !== 'false',
      maxTransactionSize: parseInt(process.env.MAX_TX_SIZE || '1232', 10),
      rateLimit: parseInt(process.env.RATE_LIMIT || '100', 10),
    },
  };

  // Validate staking parameters
  if (config.staking.minStakeAmount >= config.staking.maxStakeAmount) {
    throw new Error('MIN_STAKE_AMOUNT must be less than MAX_STAKE_AMOUNT');
  }

  if (config.staking.minDurationSeconds >= config.staking.maxDurationSeconds) {
    throw new Error('MIN_STAKE_DURATION must be less than MAX_STAKE_DURATION');
  }

  if (config.staking.baseMultiplier >= config.staking.maxMultiplier) {
    throw new Error('BASE_MULTIPLIER must be less than MAX_MULTIPLIER');
  }

  // Validate job parameters
  if (config.jobs.minBudget >= config.jobs.maxBudget) {
    throw new Error('MIN_JOB_BUDGET must be less than MAX_JOB_BUDGET');
  }

  if (config.jobs.jobTimeoutSeconds <= 0) {
    throw new Error('JOB_TIMEOUT_SECONDS must be positive');
  }

  if (config.jobs.escrowFeePercentage < 0 || config.jobs.escrowFeePercentage > 100) {
    throw new Error('ESCROW_FEE_PERCENTAGE must be between 0 and 100');
  }

  // Validate node parameters
  if (config.nodes.minReputation < 0) {
    throw new Error('MIN_REPUTATION cannot be negative');
  }

  if (config.nodes.maxConcurrentJobs <= 0) {
    throw new Error('MAX_CONCURRENT_JOBS must be positive');
  }

  if (config.nodes.slashingPercentage < 0 || config.nodes.slashingPercentage > 100) {
    throw new Error('SLASHING_PERCENTAGE must be between 0 and 100');
  }

  // Validate rewards parameters
  if (config.rewards.reflectionPercentage < 0 || config.rewards.reflectionPercentage > 100) {
    throw new Error('REFLECTION_PERCENTAGE must be between 0 and 100');
  }

  if (config.rewards.dailyRewardRate < 0) {
    throw new Error('DAILY_REWARD_RATE cannot be negative');
  }

  return config;
}

/**
 * Global configuration instance (lazy loaded)
 */
let _config: ProtocolConfig | null = null;

/**
 * Get global configuration
 */
export function getConfig(): ProtocolConfig {
  if (!_config) {
    _config = loadConfig();
  }
  return _config;
}

/**
 * Reset configuration (for testing)
 */
export function resetConfig(): void {
  _config = null;
}

/**
 * Validate configuration at startup
 */
export function validateConfig(): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  try {
    const config = loadConfig();

    // Additional validation logic here
    if (config.solanaRpcUrl.length === 0) {
      errors.push('SOLANA_RPC_URL cannot be empty');
    }

    if (!['devnet', 'testnet', 'mainnet-beta'].includes(config.network)) {
      errors.push(`Invalid network: ${config.network}`);
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  } catch (error) {
    return {
      valid: false,
      errors: [error instanceof Error ? error.message : String(error)],
    };
  }
}

/**
 * Log configuration (for debugging, sanitizes sensitive values)
 */
export function logConfiguration(config: ProtocolConfig): void {
  console.log('Protocol Configuration:');
  console.log(`  Network: ${config.network}`);
  console.log(`  RPC URL: ${config.solanaRpcUrl.substring(0, 30)}...`);
  console.log(`  Commitment: ${config.solanaCommitment}`);
  console.log(`  Staking Min/Max: ${config.staking.minStakeAmount}/${config.staking.maxStakeAmount}`);
  console.log(`  Jobs Timeout: ${config.jobs.jobTimeoutSeconds}s`);
  console.log(`  Max Concurrent Jobs: ${config.nodes.maxConcurrentJobs}`);
  console.log(`  Features: IPFS=${config.features.ipfsIntegration}, DynamicQueue=${config.features.dynamicQueue}`);
}
