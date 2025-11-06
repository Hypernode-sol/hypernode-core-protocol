/**
 * Validation and constraint checking for Hypernode Core Protocol
 * Ensures data integrity across all program modules
 */

import { getConfig } from './config';
import { PublicKey } from '@solana/web3.js';

/**
 * Validation error class
 */
export class ValidationError extends Error {
  code: string;

  constructor(message: string, code: string = 'VALIDATION_ERROR') {
    super(message);
    this.name = 'ValidationError';
    this.code = code;
  }
}

/**
 * Validate stake amount against protocol limits
 */
export function validateStakeAmount(amount: number): void {
  const config = getConfig();

  if (amount < config.staking.minStakeAmount) {
    throw new ValidationError(
      `Stake amount ${amount} is below minimum ${config.staking.minStakeAmount}`,
      'STAKE_AMOUNT_TOO_SMALL'
    );
  }

  if (amount > config.staking.maxStakeAmount) {
    throw new ValidationError(
      `Stake amount ${amount} exceeds maximum ${config.staking.maxStakeAmount}`,
      'STAKE_AMOUNT_TOO_LARGE'
    );
  }
}

/**
 * Validate stake duration against protocol limits
 */
export function validateStakeDuration(durationSeconds: number): void {
  const config = getConfig();

  if (durationSeconds < config.staking.minDurationSeconds) {
    throw new ValidationError(
      `Duration ${durationSeconds}s is below minimum ${config.staking.minDurationSeconds}s`,
      'DURATION_TOO_SHORT'
    );
  }

  if (durationSeconds > config.staking.maxDurationSeconds) {
    throw new ValidationError(
      `Duration ${durationSeconds}s exceeds maximum ${config.staking.maxDurationSeconds}s`,
      'DURATION_TOO_LONG'
    );
  }
}

/**
 * Validate both stake amount and duration
 */
export function validateStakeParameters(amount: number, durationSeconds: number): void {
  validateStakeAmount(amount);
  validateStakeDuration(durationSeconds);
}

/**
 * Validate job budget against protocol limits
 */
export function validateJobBudget(budget: number): void {
  const config = getConfig();

  if (budget < config.jobs.minBudget) {
    throw new ValidationError(
      `Job budget ${budget} is below minimum ${config.jobs.minBudget}`,
      'JOB_BUDGET_TOO_SMALL'
    );
  }

  if (budget > config.jobs.maxBudget) {
    throw new ValidationError(
      `Job budget ${budget} exceeds maximum ${config.jobs.maxBudget}`,
      'JOB_BUDGET_TOO_LARGE'
    );
  }
}

/**
 * Validate job timeout
 */
export function validateJobTimeout(timeoutSeconds: number): void {
  const config = getConfig();

  if (timeoutSeconds <= 0) {
    throw new ValidationError('Job timeout must be positive', 'INVALID_TIMEOUT');
  }

  if (timeoutSeconds > config.jobs.jobTimeoutSeconds * 10) {
    throw new ValidationError(
      `Job timeout ${timeoutSeconds}s exceeds protocol maximum ${config.jobs.jobTimeoutSeconds * 10}s`,
      'TIMEOUT_TOO_LONG'
    );
  }
}

/**
 * Validate node reputation
 */
export function validateNodeReputation(reputation: number): void {
  const config = getConfig();

  if (reputation < 0) {
    throw new ValidationError('Node reputation cannot be negative', 'INVALID_REPUTATION');
  }

  if (reputation > 10000) {
    throw new ValidationError('Node reputation cannot exceed 10000', 'REPUTATION_TOO_HIGH');
  }
}

/**
 * Validate node availability
 */
export function validateNodeAvailability(currentJobs: number, maxConcurrent: number): void {
  const config = getConfig();

  if (currentJobs < 0) {
    throw new ValidationError('Current jobs cannot be negative', 'INVALID_JOBS_COUNT');
  }

  if (maxConcurrent <= 0) {
    throw new ValidationError('Max concurrent jobs must be positive', 'INVALID_MAX_JOBS');
  }

  if (maxConcurrent > config.nodes.maxConcurrentJobs) {
    throw new ValidationError(
      `Max concurrent jobs ${maxConcurrent} exceeds protocol limit ${config.nodes.maxConcurrentJobs}`,
      'MAX_JOBS_EXCEEDED'
    );
  }

  if (currentJobs >= maxConcurrent) {
    throw new ValidationError(
      `Node is at capacity: ${currentJobs}/${maxConcurrent} jobs running`,
      'NODE_AT_CAPACITY'
    );
  }
}

/**
 * Validate Solana public key format
 */
export function validatePublicKey(key: string): void {
  try {
    new PublicKey(key);
  } catch {
    throw new ValidationError(`Invalid Solana public key: ${key}`, 'INVALID_PUBLIC_KEY');
  }
}

/**
 * Validate IPFS content hash format
 */
export function validateIPFSHash(hash: string): void {
  if (!/^Qm[a-zA-Z0-9]{44}$/.test(hash)) {
    throw new ValidationError(
      `Invalid IPFS hash format: ${hash}`,
      'INVALID_IPFS_HASH'
    );
  }
}

/**
 * Validate job definition
 */
export function validateJobDefinition(jobDef: any): void {
  if (!jobDef) {
    throw new ValidationError('Job definition cannot be null', 'NULL_JOB_DEF');
  }

  if (typeof jobDef !== 'object' || Array.isArray(jobDef)) {
    throw new ValidationError(
      'Job definition must be an object',
      'INVALID_JOB_DEF_TYPE'
    );
  }

  if (!jobDef.model || typeof jobDef.model !== 'string') {
    throw new ValidationError('Job definition must include model name', 'MISSING_MODEL');
  }

  if (!jobDef.params || typeof jobDef.params !== 'object') {
    throw new ValidationError('Job definition must include params object', 'MISSING_PARAMS');
  }

  if (jobDef.model.length > 256) {
    throw new ValidationError(
      'Model name cannot exceed 256 characters',
      'MODEL_NAME_TOO_LONG'
    );
  }
}

/**
 * Validate job result
 */
export function validateJobResult(result: any): void {
  if (typeof result !== 'object' || result === null) {
    throw new ValidationError(
      'Job result must be an object',
      'INVALID_RESULT_TYPE'
    );
  }

  if (!result.output) {
    throw new ValidationError('Job result must include output', 'MISSING_OUTPUT');
  }

  if (result.output.length > 10485760) { // 10MB
    throw new ValidationError(
      'Job result output exceeds maximum size (10MB)',
      'RESULT_TOO_LARGE'
    );
  }
}

/**
 * Validate multiplier value
 */
export function validateMultiplier(multiplier: number): void {
  const config = getConfig();

  if (multiplier < config.staking.baseMultiplier) {
    throw new ValidationError(
      `Multiplier ${multiplier} is below minimum ${config.staking.baseMultiplier}`,
      'MULTIPLIER_TOO_LOW'
    );
  }

  if (multiplier > config.staking.maxMultiplier) {
    throw new ValidationError(
      `Multiplier ${multiplier} exceeds maximum ${config.staking.maxMultiplier}`,
      'MULTIPLIER_TOO_HIGH'
    );
  }
}

/**
 * Validate reward rate
 */
export function validateRewardRate(rate: number): void {
  if (rate < 0 || rate > 100) {
    throw new ValidationError(
      `Reward rate ${rate}% must be between 0 and 100`,
      'INVALID_REWARD_RATE'
    );
  }
}

/**
 * Validate escrow fee
 */
export function validateEscrowFee(fee: number): void {
  const config = getConfig();

  if (fee < 0 || fee > 100) {
    throw new ValidationError(
      `Escrow fee ${fee}% must be between 0 and 100`,
      'INVALID_ESCROW_FEE'
    );
  }

  if (fee !== config.jobs.escrowFeePercentage) {
    throw new ValidationError(
      `Fee mismatch: expected ${config.jobs.escrowFeePercentage}%, got ${fee}%`,
      'FEE_MISMATCH'
    );
  }
}

/**
 * Validate timestamp is recent (within 5 minutes)
 */
export function validateRecentTimestamp(timestamp: number): void {
  const now = Math.floor(Date.now() / 1000);
  const timeDiff = Math.abs(now - timestamp);

  if (timeDiff > 300) { // 5 minutes
    throw new ValidationError(
      `Timestamp is too old or in the future (diff: ${timeDiff}s)`,
      'TIMESTAMP_OUT_OF_RANGE'
    );
  }
}

/**
 * Validate integer value
 */
export function validateInteger(value: any, fieldName: string): number {
  if (!Number.isInteger(value)) {
    throw new ValidationError(
      `${fieldName} must be an integer`,
      'NOT_INTEGER'
    );
  }
  return value;
}

/**
 * Validate positive integer
 */
export function validatePositiveInteger(value: any, fieldName: string): number {
  const int = validateInteger(value, fieldName);
  if (int <= 0) {
    throw new ValidationError(
      `${fieldName} must be positive`,
      'NOT_POSITIVE'
    );
  }
  return int;
}

/**
 * Validate non-negative integer
 */
export function validateNonNegativeInteger(value: any, fieldName: string): number {
  const int = validateInteger(value, fieldName);
  if (int < 0) {
    throw new ValidationError(
      `${fieldName} cannot be negative`,
      'NEGATIVE_VALUE'
    );
  }
  return int;
}

/**
 * Validate account balance is sufficient
 */
export function validateAccountBalance(
  balance: number,
  required: number,
  fieldName: string = 'Account'
): void {
  if (balance < required) {
    throw new ValidationError(
      `${fieldName} balance ${balance} is insufficient (required: ${required})`,
      'INSUFFICIENT_BALANCE'
    );
  }
}

/**
 * Validate transaction size
 */
export function validateTransactionSize(sizeBytes: number): void {
  const config = getConfig();

  if (sizeBytes > config.security.maxTransactionSize) {
    throw new ValidationError(
      `Transaction size ${sizeBytes} bytes exceeds limit ${config.security.maxTransactionSize} bytes`,
      'TRANSACTION_TOO_LARGE'
    );
  }
}

/**
 * Run all relevant validations for a stake operation
 */
export function validateStakeOperation(
  amount: number,
  duration: number,
  authority: string
): void {
  validateStakeParameters(amount, duration);
  validatePublicKey(authority);
}

/**
 * Run all relevant validations for a job submission
 */
export function validateJobSubmission(
  budget: number,
  timeout: number,
  jobDef: any,
  clientKey: string
): void {
  validateJobBudget(budget);
  validateJobTimeout(timeout);
  validateJobDefinition(jobDef);
  validatePublicKey(clientKey);
}

/**
 * Run all relevant validations for a node registration
 */
export function validateNodeRegistration(
  nodeKey: string,
  maxJobs: number,
  region: string
): void {
  validatePublicKey(nodeKey);

  if (maxJobs <= 0 || maxJobs > 1000) {
    throw new ValidationError(
      'Max concurrent jobs must be between 1 and 1000',
      'INVALID_MAX_JOBS'
    );
  }

  if (!region || region.length === 0 || region.length > 50) {
    throw new ValidationError(
      'Region must be between 1 and 50 characters',
      'INVALID_REGION'
    );
  }
}
